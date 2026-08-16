//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1356/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1356(t104818: f64, t112456: f64, t112483: f64, t112485: f64, t112487: f64, t112491: f64, t1791: f64, t24619: f64, t24726: f64, t24846: f64, t24858: f64, t29062: f64, t29083: f64, t6611: f64, t6647: f64, t6679: f64, t6683: f64, t7624: f64, t97193: f64, t97296: f64) -> f64 {
    let t116258 = 0.68598428988911579154e-2_f64 * t29062 * t6647 - 0.25724410870841842183e-2_f64 * t97193 * t24619 - 0.13719685797782315831e-1_f64 * t104818 * t6611 - 0.43445671692977333464e-1_f64 * t112456 * t1791 + t97296 + 0.45732285992607719436e-2_f64 * t29083 * t6679 + 0.91464571985215438873e-2_f64 * t29083 * t6683 - 0.28582678745379824648e-3_f64 * t7624 * t24858 + 0.28582678745379824648e-2_f64 * t7624 * t24846 - 0.17149607247227894789e-2_f64 * t7624 * t24726 + 0.17149607247227894789e-2_f64 * t112483 + 0.91464571985215438873e-2_f64 * t112485 + 0.28963781128651555642e-1_f64 * t112487 - 0.91464571985215438873e-2_f64 * t112491;
    t116258
}
