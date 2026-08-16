//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 631/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk631(t1533: f64, t4166: f64, t1004: f64, t1648: f64, t407: f64, t4146: f64, t1170: f64, t1530: f64, t2946: f64, t3029: f64, t3031: f64, t3040: f64, t3047: f64, t3048: f64, t4147: f64, t4152: f64, t4153: f64, t4159: f64, t4164: f64) -> (f64, f64, f64, f64, f64) {
    let t4167 = t4166 * t1533;
    let t4170 = t1004 * t1648;
    let t4173 = t4146 * t407;
    let t4176 = t4166 * t407;
    let t4179 = 0.26341796731742046394e1_f64 * t1530 * t4147 + t4152 - 0.65854491829355115987e0_f64 * t1170 * t4153 - 0.65854491829355115987e0_f64 * t2946 - 0.65854491829355115987e0_f64 * t3029 - 0.13170898365871023197e1_f64 * t3031 + 0.13170898365871023197e1_f64 * t1530 * t4159 + 0.65854491829355115987e0_f64 * t4164 + 0.26341796731742046394e1_f64 * t1530 * t4167 - 0.13170898365871023197e1_f64 * t4170 - t3040 + t3047 - 0.13170898365871023197e1_f64 * t3048 - 0.13170898365871023197e1_f64 * t1170 * t4173 - 0.13170898365871023197e1_f64 * t1170 * t4176;
    (t4167, t4170, t4173, t4176, t4179)
}
