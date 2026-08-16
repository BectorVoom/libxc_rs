//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1340/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1340(t12828: f64, t3259: f64, t1659: f64, t3202: f64, t3245: f64, t13223: f64, t196: f64, t197: f64, t13220: f64, t93: f64, t1206: f64, t13965: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65882 = t12828 * t3259;
    let t65899 = t1659 * t3202;
    let t65924 = t1659 * t3245;
    let t65941 = t13223 * t196 * t197;
    let t65956 = t93 * t13220;
    let t66051 = t13965 * t1206;
    (t65882, t65899, t65924, t65941, t65956, t66051)
}
