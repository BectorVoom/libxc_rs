//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 814/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk814<F: Float>(t3457: F, t6465: F, t3073: F, t1629: F, t6263: F, t5853: F, t1533: F, t1530: F, t3031: F, t3040: F, t3047: F, t4152: F, t4164: F, t4170: F, t4182: F, t4185: F, t4188: F, t4198: F, t6456: F, t6463: F) -> (F, F, F, F, F) {
    let t6466 = t6465 * t3457;
    let t6467 = t3073 * t6466;
    let t6469 = t1629 * t6263;
    let t6472 = t6465 * t5853;
    let t6475 = t6465 * t1533;
    let t6479 = -F::cast_from(0.65854491829355115987e0_f64) * t6456 + t4152 - F::cast_from(0.65854491829355115987e0_f64) * t3031 + F::cast_from(0.13170898365871023197e1_f64) * t4164 - F::cast_from(0.26341796731742046394e1_f64) * t4170 + F::cast_from(0.13170898365871023197e1_f64) * t6463 - F::cast_from(0.13170898365871023197e1_f64) * t6467 - t3040 + t3047 + F::cast_from(0.26341796731742046394e1_f64) * t1530 * t6469 - F::cast_from(0.39512695097613069591e1_f64) * t4198 * t6472 + F::cast_from(0.39512695097613069591e1_f64) * t1530 * t6475 + t4182 - F::cast_from(0.26341796731742046394e1_f64) * t4185 - t4188;
    (t6466, t6469, t6472, t6475, t6479)
}
