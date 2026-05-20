//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1569/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1569<F: Float>(t12555: F, t1756: F, t3497: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12397: F, t16706: F, t16708: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F) {
    let t16997 = t1756 * t12555;
    let t16998 = t16997 * t3497;
    let t17010 = F::cast_from(0.2283111111111111111e-1_f64) * t16710;
    let t17011 = F::cast_from(0.11415555555555555555e-1_f64) * t16712;
    let t17020 = -t12397 + F::cast_from(0.1522074074074074074e-1_f64) * t12297 + F::cast_from(0.38051851851851851851e-2_f64) * t12299 - F::cast_from(0.11415555555555555555e-1_f64) * t12301 - F::cast_from(0.57077777777777777777e-2_f64) * t12303 + F::cast_from(0.76103703703703703702e-2_f64) * t16706 + F::cast_from(0.76103703703703703701e-2_f64) * t16708 - t17010 - t17011 + F::cast_from(0.19025925925925925925e-1_f64) * t16717 - F::cast_from(0.68493333333333333331e-1_f64) * t16722 - F::cast_from(0.2283111111111111111e-1_f64) * t16727 - F::cast_from(0.11415555555555555555e-1_f64) * t16731 + F::new(0.10274e0) * t16735 + F::cast_from(0.68493333333333333332e-1_f64) * t16740 + F::cast_from(0.34246666666666666666e-1_f64) * t16744 + F::cast_from(0.17123333333333333333e-1_f64) * t16748;
    (t16998, t17020)
}
