//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2942/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2942<F: Float>(t23663: F, t914: F, t936: F, t23798: F, t945: F, t23811: F, t964: F, t41361: F, t41549: F, t51978: F, t52774: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F) -> (F, F, F, F) {
    let t78097 = t23663 * t914;
    let t78099 = F::new(1.0) * t78097 * t936;
    let t78108 = t23798 * t945;
    let t78111 = t23811 * t964;
    let t78132 = F::cast_from(0.65956790123456790123e-2_f64) * t77499 - F::cast_from(0.17808333333333333333e-1_f64) * t77503 + F::cast_from(0.5936111111111111111e-2_f64) * t77505 - F::cast_from(0.23744444444444444444e-1_f64) * t77507 + F::cast_from(0.35616666666666666667e-1_f64) * t77509 - F::cast_from(0.35616666666666666666e-1_f64) * t63276 + F::cast_from(0.11872222222222222222e-1_f64) * t63278 + t41549 + F::cast_from(0.21369999999999999999e0_f64) * t77515 - F::cast_from(0.5936111111111111111e-1_f64) * t77518 - F::new(0.32055e0) * t77521 - t52774 + F::cast_from(0.55403703703703703703e-1_f64) * t51978 + F::cast_from(0.18467901234567901234e-1_f64) * t41361 - F::cast_from(0.35616666666666666666e-1_f64) * t77527 - F::cast_from(0.35616666666666666666e-1_f64) * t77531 + F::new(0.4274e0) * t77535 - F::new(0.32055e0) * t77539 + F::new(0.10685e0) * t77543 + F::new(0.10685e0) * t77547;
    (t78099, t78108, t78111, t78132)
}
