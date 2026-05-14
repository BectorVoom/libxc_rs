//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 959/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk959<F: Float>(t1358: F, t26358: F, t2439: F, t2435: F, t7493: F, t26069: F, t26277: F, t26072: F, t7515: F, t2097: F, t25924: F, t4077: F, t2027: F, t213: F, t25921: F, t25930: F, t26294: F, t26295: F, t26302: F, t26305: F, t26309: F, t26335: F, t26338: F, t26343: F, t26347: F, t26351: F, t26356: F, t4078: F, t561: F, t7295: F, t7511: F, t7523: F, t7528: F) -> (F, F, F) {
    let t26359 = t26358 * t1358;
    let t26361 = 0.65049603595885220126e-3 * t2439 * t26359;
    let t26363 = 0.73171657588172351096e-2 * t2435 * t7493;
    let t26365 = 0.22849835011101738147e-2 * t26069 * t26277;
    let t26366 = t26072 * t7515;
    let t26371 = t25924 * t2097 * t4077;
    let t26374 = -t26294 + 0.51405703062096148812e-1 * t26295 + 0.13170898365871023197e1 * t7511 * t4078 + 0.8673628188205199462e0 * t25921 * t7528 + 0.10975748638225852664e-1 * t26302 - 0.17347256376410398924e1 * t25930 * t26305 + t26309 - 0.4336814094102599731e0 * t2027 * t26335 + 0.65854491829355115987e0 * t213 * t26338 * t561 - 0.8673628188205199462e0 * t7295 * t26343 + 0.17347256376410398924e1 * t7295 * t26347 + 0.8673628188205199462e0 * t7295 * t26351 - 0.10975748638225852664e-1 * t26356 - t26361 + t26363 - t26365 + 0.14456046980341999104e-1 * t26366 + 0.17347256376410398924e1 * t25921 * t7523 - 0.26020884564615598386e1 * t7295 * t26371;
    (t26359, t26371, t26374)
}
