//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1086/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1086<F: Float>(t25924: F, t30278: F, t1903: F, t8085: F, t7296: F, t1904: F, t213: F, t25930: F, t26238: F, t26251: F, t26263: F, t26279: F, t26294: F, t27837: F, t28781: F, t28783: F, t28796: F, t28899: F, t30227: F, t30248: F, t30252: F, t30257: F, t30262: F, t30267: F, t561: F, t6896: F, t7295: F, t7511: F, t8100: F) -> (F, F, F, F) {
    let t30279 = t25924 * t30278;
    let t30282 = t8085 * t1903;
    let t30283 = t7296 * t30282;
    let t30286 = -F::cast_from(0.8673628188205199462e0_f64) * t7295 * t30227 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t30248 * t561 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t30252 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t30257 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t30262 + F::cast_from(0.51405703062096148812e-1_f64) * t28781 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t30267 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t8100 + F::cast_from(0.13170898365871023197e1_f64) * t7511 * t6896 - F::cast_from(0.28912093960683998208e-1_f64) * t28783 - t26238 - F::cast_from(0.13170898365871023197e1_f64) * t28899 * t1904 + t26251 - t26263 - F::cast_from(0.25702851531048074406e-1_f64) * t28796 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t30279 + t26279 - t26294 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t30283;
    (t30279, t30282, t30283, t30286)
}
