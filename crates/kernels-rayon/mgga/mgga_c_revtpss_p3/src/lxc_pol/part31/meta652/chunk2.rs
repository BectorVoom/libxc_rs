//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2166/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2166(t29833: f64, t3056: f64, t7143: f64, t100723: f64, t1043: f64, t1089: f64, t19380: f64, t19520: f64, t1976: f64, t25464: f64, t25611: f64, t25658: f64, t25671: f64, t27412: f64, t27550: f64, t27609: f64, t27642: f64, t27669: f64, t29727: f64, t29731: f64, t29751: f64, t29807: f64, t29865: f64, t29871: f64, t3304: f64, t4764: f64, t6305: f64, t6393: f64, t7144: f64, t7145: f64, t7151: f64, t7153: f64, t7160: f64, t7829: f64, t93436: f64, t93498: f64, t93516: f64, t93994: f64, t94053: f64, t988: f64, t999: f64) -> f64 {
    let t107496 = t29833 * t3056 * t7143;
    let t107509 = -0.17347256376410398924e1_f64 * t27669 * t27642 * t19520 - 0.8673628188205199462e0_f64 * t25671 * t93516 * t6305 * t3304 - 0.52041769129231196772e1_f64 * t7144 * t25464 * t29751 * t988 - 0.52041769129231196772e1_f64 * t94053 * t7145 * t29871 * t988 + 0.17347256376410398924e1_f64 * t7144 * t7160 * t29731 * t988 + 0.10408353825846239354e2_f64 * t93994 * t7145 * t29871 * t999 + 0.34694512752820797848e1_f64 * t93436 * t29865 * t93498 + 0.17347256376410398924e1_f64 * t27609 * t27412 + 0.8673628188205199462e0_f64 * t7151 * t7145 * t1976 * t19380 - 0.8673628188205199462e0_f64 * t7144 * t7145 * t29807 * t988 + 0.17347256376410398924e1_f64 * t107496 * t7153 + 0.17347256376410398924e1_f64 * t25611 * t29727 * t1043 * t1089 + 0.13170898365871023197e1_f64 * t27550 * t4764 + 0.17347256376410398924e1_f64 * t100723 * t7829 - 0.65854491829355115987e0_f64 * t25658 * t6393;
    t107509
}
