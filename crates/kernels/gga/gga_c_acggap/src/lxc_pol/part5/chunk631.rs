//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 631/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk631<F: Float>(t1533: F, t4166: F, t1004: F, t1648: F, t407: F, t4146: F, t1170: F, t1530: F, t2946: F, t3029: F, t3031: F, t3040: F, t3047: F, t3048: F, t4147: F, t4152: F, t4153: F, t4159: F, t4164: F) -> (F, F, F, F, F) {
    let t4167 = t4166 * t1533;
    let t4170 = t1004 * t1648;
    let t4173 = t4146 * t407;
    let t4176 = t4166 * t407;
    let t4179 = F::new(0.26341796731742046394e1) * t1530 * t4147 + t4152 - F::new(0.65854491829355115987e0) * t1170 * t4153 - F::new(0.65854491829355115987e0) * t2946 - F::new(0.65854491829355115987e0) * t3029 - F::new(0.13170898365871023197e1) * t3031 + F::new(0.13170898365871023197e1) * t1530 * t4159 + F::new(0.65854491829355115987e0) * t4164 + F::new(0.26341796731742046394e1) * t1530 * t4167 - F::new(0.13170898365871023197e1) * t4170 - t3040 + t3047 - F::new(0.13170898365871023197e1) * t3048 - F::new(0.13170898365871023197e1) * t1170 * t4173 - F::new(0.13170898365871023197e1) * t1170 * t4176;
    (t4167, t4170, t4173, t4176, t4179)
}
