//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1991/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1991(t87753: f64, t225: f64, t26732: f64, t87776: f64, t87779: f64, t87786: f64, t10110: f64, t2597: f64, t26582: f64, t26690: f64, t2719: f64, t7830: f64, t7841: f64, t7842: f64, t82172: f64, t82174: f64, t82182: f64, t85101: f64, t855: f64, t866: f64, t87047: f64, t87050: f64, t87746: f64, t87765: f64, t87773: f64, t87784: f64, t9593: f64) -> f64 {
    let t92846 = 0.3289868133696452873e-1_f64 * t87753;
    let t92847 = t26732 * t225;
    let t92862 = 0.16449340668482264365e-1_f64 * t87776;
    let t92863 = 0.16449340668482264365e-1_f64 * t87779;
    let t92866 = 0.15352717957250113407e0_f64 * t87786;
    let t92871 = 0.16449340668482264365e-1_f64 * t87047 - 0.46058153871750340222e0_f64 * t87050 - 0.16449340668482264365e-1_f64 * t87746 - t85101 - t92846 - 2.0_f64 * t92847 * t866 - 2.0_f64 * t9593 * t7842 + 4.0_f64 * t9593 * t7830 + 4.0_f64 * t2597 * t26690 + 4.0_f64 * t2597 * t26582 - 0.39478417604357434476e0_f64 * t87765 + 0.16449340668482264365e-1_f64 * t82172 + 0.15352717957250113407e0_f64 * t82174 - 0.16449340668482264365e-1_f64 * t87773 + t92862 + t92863 - 0.16449340668482264365e-1_f64 * t82182 - 0.3289868133696452873e-1_f64 * t87784 - t92866 - 6.0_f64 * t855 * t10110 * t7841 * t2719;
    t92871
}
