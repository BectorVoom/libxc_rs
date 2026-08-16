//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1099/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1099(t15882: f64, t321: f64, t118: f64, t305: f64, t71852: f64, t71854: f64, t76141: f64, t77830: f64, t77833: f64, t77835: f64, t77836: f64, t77837: f64, t77839: f64, t77841: f64, t77843: f64, t80102: f64) -> (f64, f64) {
    let t80402 = t15882 * t321;
    let t80407 = 0.59871208509319042821e-1_f64 * t305 * t80402 - t71852 - t76141 + t71854 - t77830 - 0.39914139006212695214e-1_f64 * t118 * t80102 - t77833 - t77835 - t77836 - t77837 - t77839 + t77841 + t77843;
    (t80402, t80407)
}
