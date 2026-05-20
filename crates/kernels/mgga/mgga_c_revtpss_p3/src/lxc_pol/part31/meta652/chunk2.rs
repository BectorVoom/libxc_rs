//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2166/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2166<F: Float>(t29833: F, t3056: F, t7143: F, t100723: F, t1043: F, t1089: F, t19380: F, t19520: F, t1976: F, t25464: F, t25611: F, t25658: F, t25671: F, t27412: F, t27550: F, t27609: F, t27642: F, t27669: F, t29727: F, t29731: F, t29751: F, t29807: F, t29865: F, t29871: F, t3304: F, t4764: F, t6305: F, t6393: F, t7144: F, t7145: F, t7151: F, t7153: F, t7160: F, t7829: F, t93436: F, t93498: F, t93516: F, t93994: F, t94053: F, t988: F, t999: F) -> F {
    let t107496 = t29833 * t3056 * t7143;
    let t107509 = -F::cast_from(0.17347256376410398924e1_f64) * t27669 * t27642 * t19520 - F::cast_from(0.8673628188205199462e0_f64) * t25671 * t93516 * t6305 * t3304 - F::cast_from(0.52041769129231196772e1_f64) * t7144 * t25464 * t29751 * t988 - F::cast_from(0.52041769129231196772e1_f64) * t94053 * t7145 * t29871 * t988 + F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7160 * t29731 * t988 + F::cast_from(0.10408353825846239354e2_f64) * t93994 * t7145 * t29871 * t999 + F::cast_from(0.34694512752820797848e1_f64) * t93436 * t29865 * t93498 + F::cast_from(0.17347256376410398924e1_f64) * t27609 * t27412 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7145 * t1976 * t19380 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7145 * t29807 * t988 + F::cast_from(0.17347256376410398924e1_f64) * t107496 * t7153 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t29727 * t1043 * t1089 + F::cast_from(0.13170898365871023197e1_f64) * t27550 * t4764 + F::cast_from(0.17347256376410398924e1_f64) * t100723 * t7829 - F::cast_from(0.65854491829355115987e0_f64) * t25658 * t6393;
    t107509
}
