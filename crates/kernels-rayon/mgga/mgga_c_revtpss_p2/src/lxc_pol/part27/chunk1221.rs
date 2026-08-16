//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1221/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1221(t213: f64, t25286: f64, t251: f64, t25304: f64, t25374: f64, t10505: f64, t93172: f64, t2453: f64, t25398: f64, t10506: f64, t25314: f64, t2828: f64, t7048: f64, t7067: f64, t7070: f64, t7071: f64, t7079: f64, t886: f64, t887: f64, t93126: f64, t93151: f64, t93158: f64, t93161: f64, t93167: f64, t93175: f64, t93177: f64, t93180: f64, t93184: f64) -> (f64, f64, f64) {
    let t93186 = t213 * t25286;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93191 = t93172 * t10505;
    let t93192 = t93190 * t93191;
    let t93194 = t2453 * t25398;
    let t93195 = t93194 * t10506;
    let t93201 = -0.28912093960683998208e-1_f64 * t93151 + 0.26020884564615598386e1_f64 * t7070 * t7071 * t25286 * t886 + 0.51405703062096148814e-2_f64 * t93158 - 0.68549505033305214441e-2_f64 * t93161 - 0.13010442282307799193e1_f64 * t7067 * t25314 + 0.13010442282307799193e1_f64 * t93126 * t7079 - 0.38554277296572111609e-1_f64 * t93167 - 0.51405703062096148814e-2_f64 * t93175 - 0.68549505033305214441e-2_f64 * t93177 + 0.21684070470512998656e-1_f64 * t93180 + 0.28912093960683998208e-1_f64 * t93184 - 0.19756347548806534796e1_f64 * t93186 * t887 + 0.13709901006661042888e-1_f64 * t93192 - 0.34697458558045176417e-2_f64 * t93195 + 0.26020884564615598386e1_f64 * t7070 * t7071 * t7048 * t2828;
    (t93189, t93191, t93201)
}
