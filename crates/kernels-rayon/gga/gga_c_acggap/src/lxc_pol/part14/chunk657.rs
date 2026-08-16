//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 657/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk657(t1629: f64, t6263: f64, t5853: f64, t6465: f64, t1533: f64, t1530: f64, t3031: f64, t3040: f64, t3047: f64, t4152: f64, t4164: f64, t4170: f64, t4182: f64, t4185: f64, t4188: f64, t4198: f64, t6456: f64, t6463: f64, t6467: f64) -> f64 {
    let t6469 = t1629 * t6263;
    let t6472 = t6465 * t5853;
    let t6475 = t6465 * t1533;
    let t6479 = -0.65854491829355115987e0_f64 * t6456 + t4152 - 0.65854491829355115987e0_f64 * t3031 + 0.13170898365871023197e1_f64 * t4164 - 0.26341796731742046394e1_f64 * t4170 + 0.13170898365871023197e1_f64 * t6463 - 0.13170898365871023197e1_f64 * t6467 - t3040 + t3047 + 0.26341796731742046394e1_f64 * t1530 * t6469 - 0.39512695097613069591e1_f64 * t4198 * t6472 + 0.39512695097613069591e1_f64 * t1530 * t6475 + t4182 - 0.26341796731742046394e1_f64 * t4185 - t4188;
    t6479
}
