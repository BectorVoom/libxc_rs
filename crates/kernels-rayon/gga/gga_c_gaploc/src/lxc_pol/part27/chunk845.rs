//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 845/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk845(t1339: f64, t7892: f64, t590: f64, t4130: f64, t493: f64, t7905: f64, t1436: f64, t1441: f64, t1537: f64, t1596: f64, t2872: f64, t2877: f64, t4667: f64, t4730: f64, t4753: f64, t4781: f64, t4811: f64, t4842: f64, t567: f64, t6580: f64, t6585: f64, t6587: f64, t6591: f64, t6594: f64, t6597: f64, t6601: f64, t8176: f64, t8180: f64, t8183: f64, t8190: f64, t8196: f64) -> f64 {
    let t8199 = t1339 * t7892;
    let t8200 = t8199 * t590;
    let t8204 = t4130 * t7892 * t590;
    let t8207 = t493 * t7905;
    let t8208 = t8207 * t590;
    let t8212 = t1339 * t7905 * t590;
    let t8220 = 0.61348681526273199482e1_f64 * t567 * t8176 + 0.46011511144704899612e1_f64 * t4730 * t8180 + 0.71500979903700853338e0_f64 * t8183 * t4667 - 0.47667319935800568892e0_f64 * t2872 * t4753 + 0.35750489951850426669e0_f64 * t1596 * t2877 - 0.71500979903700853338e0_f64 * t4842 * t8190 + 0.76685851907841499352e0_f64 * t6580 - 0.76685851907841499352e0_f64 * t6585 + 0.2044956050875773316e1_f64 * t1441 * t8196 - 0.51123901271894332902e1_f64 * t1537 * t8200 + 0.30674340763136599742e1_f64 * t4781 * t8204 - 0.1022478025437886658e1_f64 * t1436 * t8208 + 0.1022478025437886658e1_f64 * t4811 * t8212 + 0.51123901271894332902e0_f64 * t6587 - 0.89376224879626066674e-1_f64 * t6591 + 0.11916829983950142223e0_f64 * t6594 + 0.11916829983950142223e0_f64 * t6597 - 0.29792074959875355558e-1_f64 * t6601;
    t8220
}
