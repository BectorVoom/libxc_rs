//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 914/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk914(t2009: f64, t2021: f64, t45524: f64, t13592: f64, t2033: f64, t549: f64, t2631: f64, t36515: f64, t787: f64, t10827: f64, t11053: f64, t9805: f64) -> (f64, f64, f64, f64) {
    let t45527 = 0.35750489951850426669e0_f64 * t2021 * t45524 * t2009;
    let t45529 = t2033 * t549 * t13592;
    let t45530 = 0.29792074959875355558e-1_f64 * t45529;
    let t45536 = 0.17875244975925213335e2_f64 * t787 * t36515 * t2631;
    let t45542 = t9805 * t11053 * t10827;
    (t45527, t45530, t45536, t45542)
}
