//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 745/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk745(t185: f64, t8769: f64, t1717: f64, t8726: f64, t8729: f64, t8732: f64, t8735: f64, t8739: f64, t8741: f64, t8746: f64, t8752: f64, t8756: f64, t8758: f64, t8761: f64, t8766: f64) -> (f64, f64) {
    let t8770 = t185 * t8769;
    let t8771 = t8770 * t1717;
    let t8773 = 0.30362304687500000001e-3_f64 * t8726 - 0.14492726735651760868e-5_f64 * t8729 - 0.14492726735651760868e-5_f64 * t8732 - 0.72463633678258804342e-6_f64 * t8735 + 0.14492726735651760868e-5_f64 * t8739 + 0.28985453471303521736e-5_f64 * t8741 + 0.25745714186718600947e-6_f64 * t8746 - 0.33199136135672468897e-7_f64 * t8752 + 0.59028064049225649701e-7_f64 * t8756 - 0.28985453471303521736e-5_f64 * t8758 + 0.29518907335069444446e-5_f64 * t8761 - 0.33765185592488808582e-6_f64 * t8766 - 0.77294542590142724635e-6_f64 * t8771;
    (t8770, t8773)
}
