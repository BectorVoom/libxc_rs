//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1354/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1354<F: Float>(t10318: F, t4360: F, t4667: F, t10319: F, t4753: F, t2413: F, t26122: F, t26726: F, t901: F, t26822: F, t10315: F, t20445: F) -> (F, F, F, F, F, F) {
    let t35136 = F::cast_from(0.71500979903700853338e0_f64) * t4360 * t10318 * t4667;
    let t35138 = F::cast_from(0.47667319935800568892e0_f64) * t10319 * t4753;
    let t35140 = F::cast_from(0.21450293971110256002e1_f64) * t26122 * t2413;
    let t35141 = t26726 * t901;
    let t35142 = F::cast_from(0.29792074959875355558e-1_f64) * t35141;
    let t35143 = t26822 * t901;
    let t35144 = F::cast_from(0.14896037479937677779e-1_f64) * t35143;
    let t35146 = F::cast_from(0.14300195980740170668e1_f64) * t20445 * t10315;
    (t35136, t35138, t35140, t35142, t35144, t35146)
}
