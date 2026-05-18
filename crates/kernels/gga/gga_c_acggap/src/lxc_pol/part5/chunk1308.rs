//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1308/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1308<F: Float>(t14230: F, t3073: F, t6465: F, t4180: F, t6469: F, t377: F, t6507: F, t119: F, t1247: F, t14501: F, t14503: F, t151: F, t1530: F, t1629: F, t182: F, t1839: F, t19133: F, t19144: F, t19149: F, t19152: F, t22127: F, t23821: F, t24340: F, t2925: F, t6551: F, t930: F) -> F {
    let t24359 = t3073 * t6465 * t14230;
    let t24361 = t4180 * t6469;
    let t24363 = t377 * t6507;
    let t24368 = F::new(0.65854491829355115987e0) * t119 * t182 * t24340 - F::new(0.26341796731742046394e1) * t19133 - F::new(0.65854491829355115987e0) * t151 * t2925 * t1839 + t14501 - F::new(0.65854491829355115987e0) * t151 * t6551 * t930 + F::new(0.26341796731742046394e1) * t151 * t1247 * t22127 + F::new(0.13170898365871023197e1) * t14503 + F::new(0.26341796731742046394e1) * t1530 * t1629 * t23821 - F::new(0.13170898365871023197e1) * t24359 + F::new(0.52683593463484092788e1) * t24361 - F::new(0.26341796731742046394e1) * t24363 - F::new(0.26341796731742046394e1) * t19144 + F::new(0.52683593463484092788e1) * t19149 - F::new(0.79025390195226139182e1) * t19152;
    t24368
}
