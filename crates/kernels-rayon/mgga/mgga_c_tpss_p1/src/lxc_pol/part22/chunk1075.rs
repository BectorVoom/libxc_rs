//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1075/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1075(t2976: f64, t4205: f64, t1089: f64, t3009: f64, t4198: f64, t1542: f64, t9172: f64, t2975: f64, t9176: f64, t2973: f64, t4180: f64, t1082: f64) -> (f64, f64, f64, f64) {
    let t11819 = t4205 * t2976;
    let t11821 = 0.35089341735807877242e1_f64 * t1089 * t11819;
    let t11823 = 0.23392894490538584828e1_f64 * t3009 * t4198;
    let t11824 = t9172 * t1542;
    let t11825 = t9176 * t2975;
    let t11826 = t11824 * t11825;
    let t11828 = 0.10254018858216406658e4_f64 * t1089 * t11826;
    let t11829 = t2973 * t4180;
    let t11830 = t11829 * t1082;
    (t11821, t11823, t11828, t11830)
}
