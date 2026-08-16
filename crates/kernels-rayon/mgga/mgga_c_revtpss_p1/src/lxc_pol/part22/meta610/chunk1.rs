//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2513/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2513(t359: f64, t6343: f64, t999: f64, t1086: f64, t6235: f64, t1647: f64, t4995: f64, t3153: f64, t6299: f64) -> (f64, f64, f64, f64, f64) {
    let t19556 = t359 * t6343;
    let t19557 = t19556 * t999;
    let t19566 = t6235 * t1086;
    let t19569 = t1647 * t4995;
    let t19572 = t6299 * t3153;
    (t19556, t19557, t19566, t19569, t19572)
}
