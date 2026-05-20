//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3460/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3460<F: Float>(t341: F, t65012: F, t65026: F, t65040: F, t65054: F, t15648: F, t1668: F, t3059: F, t6258: F, t3057: F, t3140: F, t1035: F, t1082: F, t1089: F, t1093: F, t11940: F, t12146: F, t15670: F, t15837: F, t16402: F, t16436: F, t16488: F, t1651: F, t16584: F, t19453: F, t19584: F, t19856: F, t20136: F, t3151: F, t3204: F, t3278: F, t3287: F, t357: F, t378: F, t381: F, t43350: F, t4743: F, t4961: F, t4977: F, t4980: F, t4984: F, t4995: F, t4999: F, t5004: F, t54695: F, t55685: F, t55805: F, t55948: F, t64891: F) -> (F, F, F, F, F) {
    let t65057 = (t65012 + t65026 + t65040 + t65054) * t341;
    let t65060 = t15648 * t1668;
    let t65071 = t6258 * t3059;
    let t65096 = t3057 * t3140;
    let t65102 = F::cast_from(0.13170898365871023197e1_f64) * t19856 * t1093 + F::cast_from(0.65854491829355115987e0_f64) * t65057 * t381 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t65060 * t1089 + F::cast_from(0.26341796731742046394e1_f64) * t3204 * t5004 * t15837 - F::cast_from(0.26341796731742046394e1_f64) * t12146 * t20136 + F::cast_from(0.26341796731742046394e1_f64) * t15670 * t16402 - F::cast_from(0.39512695097613069591e1_f64) * t11940 * t1082 * t65071 + F::cast_from(0.13170898365871023197e1_f64) * t55948 * t19453 + F::cast_from(0.52683593463484092788e1_f64) * t54695 * t4961 + F::cast_from(0.26341796731742046394e1_f64) * t3278 * t19584 + F::cast_from(0.52683593463484092788e1_f64) * t4743 * t4980 * t4984 - F::cast_from(0.26341796731742046394e1_f64) * t4743 * t4995 * t4999 - F::cast_from(0.65854491829355115987e0_f64) * t55805 * t64891 * t43350 * t3151 * t357 - F::cast_from(0.26341796731742046394e1_f64) * t55685 * t4977 - F::cast_from(0.13170898365871023197e1_f64) * t16584 * t16488 - F::cast_from(0.52683593463484092788e1_f64) * t65096 * t1035 * t378 * t1651 * t16436;
    (t65057, t65060, t65071, t65096, t65102)
}
