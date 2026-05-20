//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2165/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2165<F: Float>(t106655: F, t994: F, t106719: F, t107226: F, t1089: F, t1096: F, t1651: F, t1696: F, t19477: F, t25591: F, t25640: F, t25671: F, t25681: F, t27419: F, t27557: F, t27679: F, t27684: F, t29727: F, t29759: F, t29807: F, t29826: F, t29852: F, t3318: F, t4772: F, t5015: F, t6299: F, t6305: F, t7144: F, t7145: F, t7147: F, t7159: F, t7160: F, t7167: F, t7168: F, t7817: F, t93490: F, t93983: F, t93984: F, t94016: F, t988: F, t99666: F) -> F {
    let t107435 = t994 * t106655;
    let t107457 = -F::cast_from(0.34694512752820797848e1_f64) * t27419 * t27684 + F::cast_from(0.17347256376410398924e1_f64) * t27419 * t27557 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t7817 * t4772 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t27679 * t1651 - F::cast_from(0.52041769129231196772e1_f64) * t94016 * t29759 * t106719 + F::cast_from(0.4336814094102599731e0_f64) * t93490 * t29852 + F::cast_from(0.4336814094102599731e0_f64) * t25671 * t25681 * t6305 * t3318 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t29727 * t988 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t7817 * t5015 - F::cast_from(0.8673628188205199462e0_f64) * t107435 * t7147 - F::cast_from(0.13170898365871023197e1_f64) * t99666 * t1696 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7160 * t29807 * t1096 - F::cast_from(0.4336814094102599731e0_f64) * t25640 * t29826 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t25681 * t6299 * t1089 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t7168 * t19477 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t93983 * t107226 * t93984;
    t107457
}
