//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2161/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2161<F: Float>(t1035: F, t29807: F, t29834: F, t7166: F, t1976: F, t6305: F, t3153: F, t6235: F, t100431: F, t100658: F, t1043: F, t1089: F, t1097: F, t1668: F, t1695: F, t1696: F, t19342: F, t19579: F, t20151: F, t25461: F, t25473: F, t25611: F, t27422: F, t27579: F, t27687: F, t29728: F, t29732: F, t29844: F, t6245: F, t7102: F, t7144: F, t7159: F, t7160: F, t7167: F, t7170: F, t93884: F, t93897: F, t99629: F, t99881: F) -> (F, F, F) {
    let t107207 = t1035 * t29807;
    let t107212 = t29834 * t7166;
    let t107225 = t1976 * t6305;
    let t107226 = t107225 * t3153;
    let t107240 = t6235 * t1976;
    let t107257 = -F::cast_from(0.4336814094102599731e0_f64) * t7167 * t107207 * t1043 * t1089 - F::cast_from(0.8673628188205199462e0_f64) * t107212 * t7170 - F::cast_from(0.8673628188205199462e0_f64) * t7167 * t100431 * t1668 * t1089 - F::cast_from(0.13170898365871023197e1_f64) * t99881 * t1696 + F::cast_from(0.13170898365871023197e1_f64) * t93884 * t6245 + F::cast_from(0.34694512752820797848e1_f64) * t100658 * t29844 - F::cast_from(0.8673628188205199462e0_f64) * t93897 * t107226 * t19579 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t27687 * t1668 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t27579 * t1668 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t25461 * t29728 - F::cast_from(0.65854491829355115987e0_f64) * t107240 * t1097 + F::cast_from(0.8673628188205199462e0_f64) * t25473 * t29732 - F::cast_from(0.13170898365871023197e1_f64) * t7102 * t19342 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t27422 * t1695 - F::cast_from(0.13170898365871023197e1_f64) * t99629 * t1696 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7160 * t1976 * t20151;
    (t107225, t107226, t107257)
}
