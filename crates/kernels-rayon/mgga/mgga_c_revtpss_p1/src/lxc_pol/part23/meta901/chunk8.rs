//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2875/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2875(t213: f64, t23359: f64, t1580: f64, t18663: f64, t18785: f64, t18800: f64, t225: f64, t23413: f64, t257: f64, t41078: f64, t41118: f64, t4474: f64, t4534: f64, t51733: f64, t51742: f64, t51756: f64, t63085: f64, t63091: f64, t63094: f64, t63099: f64, t63103: f64, t63109: f64, t77151: f64, t865: f64, t886: f64, t887: f64) -> f64 {
    let t77316 = t213 * t23359;
    let t77326 = -0.19756347548806534796e1_f64 * t18800 * t4534 + 0.78059524315062264151e-2_f64 * t51733 - 0.19756347548806534796e1_f64 * t63103 * t1580 - 0.29272321618148349057e-1_f64 * t63085 + t51742 - 0.65854491829355115984e-1_f64 * t63091 + 0.11708928647259339623e0_f64 * t63094 + 0.7805952431506226415e-1_f64 * t63099 + 0.15805078039045227836e2_f64 * t865 * t41078 * t23413 * t886 - 0.11853808529283920877e2_f64 * t4474 * t18663 - 0.19756347548806534796e1_f64 * t4474 * t18785 - 0.65854491829355115987e0_f64 * t77316 * t887 - 0.58544643236296698113e-1_f64 * t63109 + 0.11044544084478153697e-3_f64 * t41118 + 0.65854491829355115987e0_f64 * t213 * t77151 * t225 * t257 - 0.39029762157531132076e-2_f64 * t51756;
    t77326
}
