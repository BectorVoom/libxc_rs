//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 853/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk853<F: Float>(t2131: F, t2147: F, t309: F, t8436: F, t2351: F, t7924: F, t463: F, t8422: F, t545: F, t7923: F, t621: F, t2331: F, t310: F, t464: F, t7984: F, t8998: F) -> (F, F, F, F, F, F, F) {
    let t33516 = 0.34694512752820797848e1 * t2131 * t2147 * t8436 * t309;
    let t33518 = t7924 * t2351;
    let t33523 = 0.34694512752820797848e1 * t2131 * t2147 * t8422 * t463;
    let t33524 = t7923 * t545;
    let t33525 = t33524 * t621;
    let t33527 = t310 * t2331;
    let t33529 = 0.13170898365871023197e1 * t33527 * t464;
    let t33533 = 0.17347256376410398924e1 * t8998 * t7984;
    (t33516, t33518, t33523, t33525, t33527, t33529, t33533)
}
