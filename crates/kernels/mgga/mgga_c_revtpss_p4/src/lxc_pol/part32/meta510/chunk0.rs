//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1801/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1801<F: Float>(t30286: F, t30312: F, t532: F, t1450: F, t2071: F, t29591: F, t26550: F, t29682: F, t1579: F, t7997: F, t7071: F, t1580: F, t25391: F, t26437: F, t26439: F, t26508: F, t26521: F, t27199: F, t28315: F, t28317: F, t28352: F, t28361: F, t28366: F, t28369: F, t28371: F, t28374: F, t28391: F, t28394: F, t6049: F, t6072: F, t7070: F, t7403: F, t8012: F) -> (F, F, F, F, F, F, F, F) {
    let t30313 = t30286 + t30312;
    let t30314 = t532 * t30313;
    let t30315 = t30314 * t1450;
    let t30317 = t2071 * t29591;
    let t30337 = t26550 * t29682;
    let t30341 = t7997 * t1579;
    let t30342 = t7071 * t30341;
    let t30355 = -t26437 + t26439 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t8012 - F::cast_from(0.28912093960683998208e-1_f64) * t28315 + F::cast_from(0.51405703062096148812e-1_f64) * t28317 - F::cast_from(0.65854491829355115987e0_f64) * t7403 * t6072 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t30337 - F::cast_from(0.25702851531048074406e-1_f64) * t28352 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t30342 + F::cast_from(0.14456046980341999104e-1_f64) * t28361 - F::cast_from(0.25702851531048074406e-1_f64) * t28366 - F::cast_from(0.14456046980341999104e-1_f64) * t28369 + F::cast_from(0.25702851531048074406e-1_f64) * t28371 + F::cast_from(0.13170898365871023197e1_f64) * t7403 * t6049 + F::cast_from(0.19514881078765566038e-1_f64) * t28374 + F::cast_from(0.10975748638225852664e-1_f64) * t28391 + t26508 + t26521 - F::cast_from(0.13170898365871023197e1_f64) * t28394 * t1580;
    (t30313, t30314, t30315, t30317, t30337, t30341, t30342, t30355)
}
