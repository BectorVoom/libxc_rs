//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 189/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk189(t241: f64, t835: f64, t244: f64, t248: f64, t238: f64, t234: f64, t236: f64, t240: f64, t812: f64, t200: f64, t243: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t836 = t835 * t241;
    let t838 = t836 * t244 * t248;
    let t840 = 7.0_f64 / 4608.0_f64 * t238 * t838;
    let t841 = t234 * t236;
    let t842 = t841 * t240;
    let t843 = t812 * t842;
    let t845 = 1.0_f64 / t243 / t200;
    let t847 = t241 * t845 * t67;
    (t836, t838, t840, t841, t842, t843, t845, t847)
}
