//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1547/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1547(t359: f64, t6343: f64, t999: f64, t1086: f64, t6235: f64, t1647: f64, t4995: f64, t3153: f64, t6299: f64, t4983: f64, t4998: f64, t19482: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19556 = t359 * t6343;
    let t19557 = t19556 * t999;
    let t19566 = t6235 * t1086;
    let t19569 = t1647 * t4995;
    let t19572 = t6299 * t3153;
    let t19573 = t19572 * t4983;
    let t19576 = t19572 * t4998;
    let t19579 = t19482 * t999;
    (t19557, t19566, t19569, t19572, t19573, t19576, t19579)
}
