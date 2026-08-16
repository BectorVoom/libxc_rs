//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 582/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk582(t407: f64, t4237: f64, t3457: f64, t406: f64, t1629: f64, t3073: f64, t1647: f64, t864: f64, t1035: f64, t151: f64, t3075: f64, t3078: f64, t3081: f64, t3091: f64, t3104: f64, t3827: f64, t3830: f64, t4228: f64, t4230: f64, t4231: f64, t4234: f64, t4235: f64) -> (f64, f64) {
    let t4238 = t4237 * t407;
    let t4241 = t3457 * t406;
    let t4242 = t1629 * t4241;
    let t4244 = 0.26341796731742046394e1_f64 * t3073 * t4242;
    let t4245 = t1647 * t864;
    let t4246 = t1035 * t4245;
    let t4249 = -0.13170898365871023197e1_f64 * t3075 + 0.26341796731742046395e1_f64 * t3078 + 0.65854491829355115987e0_f64 * t3081 - 0.26341796731742046394e1_f64 * t3091 - t4228 - t4230 - 0.65854491829355115987e0_f64 * t4231 - t4234 - t3104 + 0.65854491829355115987e0_f64 * t4235 - 0.13170898365871023197e1_f64 * t151 * t4238 - t4244 + 0.13170898365871023197e1_f64 * t4246 - t3827 - 0.65854491829355115987e0_f64 * t3830;
    (t4241, t4249)
}
