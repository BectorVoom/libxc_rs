//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1007/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1007(t5: f64, t2170: f64, t7953: f64, t8142: f64, t8441: f64, t8621: f64, t33359: f64, t33363: f64, t33370: f64, t33609: f64, t33613: f64, t33617: f64, t33625: f64, t8737: f64, t8913: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t34485 = t2170 * t7953;
    let t34866 = t8441 * t8142;
    let t34867 = t8621 * t34866;
    let t34873 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t33609 * t8913 - 5.0_f64 / 24.0_f64 * t33359 * t33613 - 5.0_f64 / 36.0_f64 * t33363 * t33617 + 5.0_f64 / 72.0_f64 * t8737 * t34867 + 5.0_f64 / 72.0_f64 * t33370 * t33625);
    (t34485, t34866, t34867, t34873)
}
