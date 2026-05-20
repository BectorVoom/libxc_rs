//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2169/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2169<F: Float>(t1647: F, t7810: F, t1078: F, t1982: F, t3140: F, t6343: F, t100702: F, t1043: F, t1089: F, t1097: F, t1651: F, t1652: F, t1696: F, t19381: F, t1986: F, t20112: F, t25591: F, t25695: F, t25699: F, t27415: F, t27422: F, t27433: F, t27445: F, t27621: F, t27627: F, t27661: F, t29747: F, t29866: F, t29871: F, t6235: F, t6244: F, t6259: F, t7102: F, t7135: F, t7137: F, t7145: F, t7170: F, t94122: F, t99675: F, t999: F, t99940: F) -> F {
    let t107629 = t1647 * t7810;
    let t107636 = t1982 * t6343 * t3140 * t1078;
    let t107649 = F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t27422 * t1651 - F::cast_from(0.26020884564615598386e1_f64) * t25699 * t7145 * t7135 * t6244 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t29747 * t999 + F::cast_from(0.34694512752820797848e1_f64) * t27415 * t29866 - F::cast_from(0.65854491829355115987e0_f64) * t25695 * t6259 - F::cast_from(0.65854491829355115987e0_f64) * t7102 * t19381 - F::cast_from(0.8673628188205199462e0_f64) * t27621 * t27627 - F::cast_from(0.4336814094102599731e0_f64) * t1982 * t20112 * t1986 - F::cast_from(0.17347256376410398924e1_f64) * t27661 * t27445 - F::cast_from(0.13170898365871023197e1_f64) * t107629 * t1097 + F::cast_from(0.65854491829355115987e0_f64) * t6235 * t7137 - F::cast_from(0.4336814094102599731e0_f64) * t107636 * t7170 - F::cast_from(0.17347256376410398924e1_f64) * t99675 * t27433 - F::cast_from(0.26020884564615598386e1_f64) * t94122 * t29871 * t1043 * t1089 - F::cast_from(0.13170898365871023197e1_f64) * t99940 * t1652 - F::cast_from(0.13170898365871023197e1_f64) * t100702 * t1696;
    t107649
}
