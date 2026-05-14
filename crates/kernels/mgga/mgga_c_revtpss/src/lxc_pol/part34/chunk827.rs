//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 827/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk827<F: Float>(t2926: F, t6141: F, t342: F, t6343: F, t6234: F, t993: F, t225: F, t3011: F, t6205: F, t3302: F, t357: F, t3153: F, t6305: F, t359: F, t1086: F, t6235: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19330 = t6141 * t2926;
    let t19351 = t342 * t6343;
    let t19462 = t6234 * t993;
    let t19463 = t19462 * t225;
    let t19467 = t3011 * t6205;
    let t19482 = t3302 * t357;
    let t19501 = t6305 * t3153;
    let t19556 = t359 * t6343;
    let t19566 = t6235 * t1086;
    (t19330, t19351, t19462, t19463, t19467, t19482, t19501, t19556, t19566)
}
