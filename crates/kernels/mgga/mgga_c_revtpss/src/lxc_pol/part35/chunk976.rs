//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 976/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk976<F: Float>(t225: F, t30247: F, t26304: F, t30105: F, t1882: F, t543: F, t8085: F, t7301: F, t2097: F, t6843: F, t30225: F, t6895: F, t25924: F, t1903: F, t7296: F, t1904: F, t213: F, t25930: F, t26238: F, t26251: F, t26263: F, t26279: F, t26294: F, t27837: F, t28781: F, t28783: F, t28796: F, t28899: F, t30227: F, t561: F, t6896: F, t7295: F, t7511: F, t8100: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t30248 = t30247 * t225;
    let t30252 = t26304 * t30105;
    let t30256 = t8085 * t1882 * t543;
    let t30257 = t7301 * t30256;
    let t30261 = t2097 * t6843 * t543;
    let t30262 = t7301 * t30261;
    let t30266 = t30225 * t543;
    let t30267 = t7301 * t30266;
    let t30278 = t2097 * t6895;
    let t30279 = t25924 * t30278;
    let t30282 = t8085 * t1903;
    let t30283 = t7296 * t30282;
    let t30286 = -0.8673628188205199462e0 * t7295 * t30227 + 0.65854491829355115987e0 * t213 * t30248 * t561 - 0.17347256376410398924e1 * t25930 * t30252 + 0.8673628188205199462e0 * t7295 * t30257 + 0.4336814094102599731e0 * t7295 * t30262 + 0.51405703062096148812e-1 * t28781 + 0.4336814094102599731e0 * t7295 * t30267 + 0.8673628188205199462e0 * t27837 * t8100 + 0.13170898365871023197e1 * t7511 * t6896 - 0.28912093960683998208e-1 * t28783 - t26238 - 0.13170898365871023197e1 * t28899 * t1904 + t26251 - t26263 - 0.25702851531048074406e-1 * t28796 - 0.26020884564615598386e1 * t7295 * t30279 + t26279 - t26294 + 0.17347256376410398924e1 * t7295 * t30283;
    (t30248, t30252, t30256, t30257, t30261, t30262, t30266, t30267, t30278, t30279, t30282, t30283, t30286)
}
