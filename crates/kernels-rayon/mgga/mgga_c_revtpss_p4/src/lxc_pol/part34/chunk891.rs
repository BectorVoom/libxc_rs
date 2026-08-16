//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 891/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk891(t3302: f64, t357: f64, t3153: f64, t6305: f64, t359: f64, t6343: f64, t1086: f64, t6235: f64, t6299: f64, t73: f64, t1065: f64, t6244: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19482 = t3302 * t357;
    let t19501 = t6305 * t3153;
    let t19556 = t359 * t6343;
    let t19566 = t6235 * t1086;
    let t19572 = t6299 * t3153;
    let t19611 = t6299 * t73;
    let t19649 = t1065 * t6244;
    (t19482, t19501, t19556, t19566, t19572, t19611, t19649)
}
