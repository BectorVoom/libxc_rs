//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1126/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1126(t157: f64, t20084: f64, t20122: f64, t377: f64, t6495: f64, t12360: f64, t151: f64, t1530: f64, t1533: f64, t1844: f64, t19005: f64, t19015: f64, t19023: f64, t19026: f64, t19029: f64, t19032: f64, t19807: f64, t19894: f64, t19898: f64, t2925: f64, t456: f64, t6461: f64, t945: f64) -> (f64, f64) {
    let t20124 = (t20084 + t20122) * t157;
    let t20128 = t377 * t6495;
    let t20134 = -0.65854491829355115987e0_f64 * t151 * t2925 * t1844 + 0.26341796731742046394e1_f64 * t19005 - 0.26341796731742046394e1_f64 * t19894 - 0.26341796731742046394e1_f64 * t19015 - 0.52683593463484092788e1_f64 * t19898 + 0.79025390195226139182e1_f64 * t1530 * t6461 * t945 + 0.52683593463484092788e1_f64 * t1530 * t19807 * t1533 - 0.65854491829355115987e0_f64 * t151 * t456 * t20124 - t12360 - 0.13170898365871023197e1_f64 * t20128 + 0.26341796731742046394e1_f64 * t19023 + 0.26341796731742046394e1_f64 * t19026 - 0.52683593463484092788e1_f64 * t19029 + 0.26341796731742046394e1_f64 * t19032;
    (t20124, t20134)
}
