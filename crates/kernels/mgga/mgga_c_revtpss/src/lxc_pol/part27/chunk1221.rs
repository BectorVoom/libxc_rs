//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1221/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1221<F: Float>(t213: F, t25286: F, t251: F, t25304: F, t25374: F, t10505: F, t93172: F, t2453: F, t25398: F, t10506: F, t25314: F, t2828: F, t7048: F, t7067: F, t7070: F, t7071: F, t7079: F, t886: F, t887: F, t93126: F, t93151: F, t93158: F, t93161: F, t93167: F, t93175: F, t93177: F, t93180: F, t93184: F) -> (F, F, F) {
    let t93186 = t213 * t25286;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93191 = t93172 * t10505;
    let t93192 = t93190 * t93191;
    let t93194 = t2453 * t25398;
    let t93195 = t93194 * t10506;
    let t93201 = -F::cast_from(0.28912093960683998208e-1_f64) * t93151 + F::cast_from(0.26020884564615598386e1_f64) * t7070 * t7071 * t25286 * t886 + F::cast_from(0.51405703062096148814e-2_f64) * t93158 - F::cast_from(0.68549505033305214441e-2_f64) * t93161 - F::cast_from(0.13010442282307799193e1_f64) * t7067 * t25314 + F::cast_from(0.13010442282307799193e1_f64) * t93126 * t7079 - F::cast_from(0.38554277296572111609e-1_f64) * t93167 - F::cast_from(0.51405703062096148814e-2_f64) * t93175 - F::cast_from(0.68549505033305214441e-2_f64) * t93177 + F::cast_from(0.21684070470512998656e-1_f64) * t93180 + F::cast_from(0.28912093960683998208e-1_f64) * t93184 - F::cast_from(0.19756347548806534796e1_f64) * t93186 * t887 + F::cast_from(0.13709901006661042888e-1_f64) * t93192 - F::cast_from(0.34697458558045176417e-2_f64) * t93195 + F::cast_from(0.26020884564615598386e1_f64) * t7070 * t7071 * t7048 * t2828;
    (t93189, t93191, t93201)
}
