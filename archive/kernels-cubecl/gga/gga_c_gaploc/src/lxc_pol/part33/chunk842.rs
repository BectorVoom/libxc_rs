//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 842/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk842<F: Float>(t1339: F, t7892: F, t590: F, t4130: F, t493: F, t7905: F, t1436: F, t1441: F, t1537: F, t1596: F, t2872: F, t2877: F, t4667: F, t4730: F, t4753: F, t4781: F, t4811: F, t4842: F, t567: F, t6580: F, t6585: F, t6587: F, t6591: F, t6594: F, t6597: F, t6601: F, t8176: F, t8180: F, t8183: F, t8190: F, t8196: F) -> F {
    let t8199 = t1339 * t7892;
    let t8200 = t8199 * t590;
    let t8204 = t4130 * t7892 * t590;
    let t8207 = t493 * t7905;
    let t8208 = t8207 * t590;
    let t8212 = t1339 * t7905 * t590;
    let t8220 = F::cast_from(0.61348681526273199482e1_f64) * t567 * t8176 + F::cast_from(0.46011511144704899612e1_f64) * t4730 * t8180 + F::cast_from(0.71500979903700853338e0_f64) * t8183 * t4667 - F::cast_from(0.47667319935800568892e0_f64) * t2872 * t4753 + F::cast_from(0.35750489951850426669e0_f64) * t1596 * t2877 - F::cast_from(0.71500979903700853338e0_f64) * t4842 * t8190 + F::cast_from(0.76685851907841499352e0_f64) * t6580 - F::cast_from(0.76685851907841499352e0_f64) * t6585 + F::cast_from(0.2044956050875773316e1_f64) * t1441 * t8196 - F::cast_from(0.51123901271894332902e1_f64) * t1537 * t8200 + F::cast_from(0.30674340763136599742e1_f64) * t4781 * t8204 - F::cast_from(0.1022478025437886658e1_f64) * t1436 * t8208 + F::cast_from(0.1022478025437886658e1_f64) * t4811 * t8212 + F::cast_from(0.51123901271894332902e0_f64) * t6587 - F::cast_from(0.89376224879626066674e-1_f64) * t6591 + F::cast_from(0.11916829983950142223e0_f64) * t6594 + F::cast_from(0.11916829983950142223e0_f64) * t6597 - F::cast_from(0.29792074959875355558e-1_f64) * t6601;
    t8220
}
