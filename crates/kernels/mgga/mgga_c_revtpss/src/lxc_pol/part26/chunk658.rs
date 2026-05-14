//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 658/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk658<F: Float>(t7058: F, t7407: F, t7064: F, t2061: F, t886: F, t7071: F, t231: F, t836: F, t7076: F, t233: F, t7398: F, t1957: F, t1956: F, t2067: F, t213: F, t257: F, t7067: F, t7070: F, t7387: F, t7390: F, t7399: F, t7403: F, t887: F) -> (F, F, F, F, F, F, F) {
    let t7409 = 0.72280234901709995518e-2 * t7058 * t7407;
    let t7411 = 0.12851425765524037203e-1 * t7064 * t7407;
    let t7414 = t2061 * t886;
    let t7415 = t7071 * t7414;
    let t7419 = t2061 * t836 * t231;
    let t7420 = t7076 * t7419;
    let t7423 = t233 * t7398;
    let t7424 = t1957 * t7423;
    let t7427 = -t7387 + t7390 + 0.65854491829355115987e0 * t213 * t7399 * t257 - 0.65854491829355115987e0 * t7403 * t887 + t7409 - t7411 - 0.4336814094102599731e0 * t7067 * t2067 + 0.8673628188205199462e0 * t7070 * t7415 + 0.4336814094102599731e0 * t7070 * t7420 - 0.4336814094102599731e0 * t1956 * t7424;
    (t7414, t7415, t7419, t7420, t7423, t7424, t7427)
}
