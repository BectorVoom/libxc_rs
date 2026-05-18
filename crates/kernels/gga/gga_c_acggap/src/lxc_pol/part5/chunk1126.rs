//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1126/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1126<F: Float>(t157: F, t20084: F, t20122: F, t377: F, t6495: F, t12360: F, t151: F, t1530: F, t1533: F, t1844: F, t19005: F, t19015: F, t19023: F, t19026: F, t19029: F, t19032: F, t19807: F, t19894: F, t19898: F, t2925: F, t456: F, t6461: F, t945: F) -> (F, F) {
    let t20124 = (t20084 + t20122) * t157;
    let t20128 = t377 * t6495;
    let t20134 = -F::new(0.65854491829355115987e0) * t151 * t2925 * t1844 + F::new(0.26341796731742046394e1) * t19005 - F::new(0.26341796731742046394e1) * t19894 - F::new(0.26341796731742046394e1) * t19015 - F::new(0.52683593463484092788e1) * t19898 + F::new(0.79025390195226139182e1) * t1530 * t6461 * t945 + F::new(0.52683593463484092788e1) * t1530 * t19807 * t1533 - F::new(0.65854491829355115987e0) * t151 * t456 * t20124 - t12360 - F::new(0.13170898365871023197e1) * t20128 + F::new(0.26341796731742046394e1) * t19023 + F::new(0.26341796731742046394e1) * t19026 - F::new(0.52683593463484092788e1) * t19029 + F::new(0.26341796731742046394e1) * t19032;
    (t20124, t20134)
}
