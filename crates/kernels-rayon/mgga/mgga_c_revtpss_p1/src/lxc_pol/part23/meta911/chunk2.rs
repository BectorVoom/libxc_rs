//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2928/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2928(t77681: f64, t77705: f64, t77732: f64, t77747: f64, t77801: f64, t77824: f64, t77846: f64, t77860: f64, t11404: f64, t15343: f64, t19156: f64, t19167: f64, t23706: f64, t23717: f64, t23723: f64, t41756: f64, t41779: f64, t4685: f64, t4708: f64, t52809: f64, t52820: f64, t6158: f64, t6177: f64, t6206: f64, t77639: f64, t77641: f64, t77643: f64, t77645: f64, t77647: f64, t77657: f64, t965: f64, t973: f64) -> (f64, f64) {
    let t77863 = t77681 + t77705 + t77732 + t77747 + t77801 + t77824 + t77846 + t77860;
    let t77873 = -t77639 - t77641 - t77643 + t77645 - t77647 + 0.17544670867903938621e1_f64 * t19156 * t4708 + 0.17544670867903938621e1_f64 * t15343 * t6206 + 0.17544670867903938621e1_f64 * t4685 * t19167 - t77657 - 6.0_f64 * t52809 * t6158 + 6.0_f64 * t11404 * t23706 + 0.5848223622634646207e0_f64 * t965 * t77863 * t973 + 0.10254018858216406658e4_f64 * t41756 * t23717 + 0.96491876992155210402e2_f64 * t52820 * t6177 - 0.19298375398431042081e3_f64 * t41779 * t23723;
    (t77863, t77873)
}
