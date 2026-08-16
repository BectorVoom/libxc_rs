//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2143/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2143(t355: f64, t4910: f64, t1976: f64, t6299: f64, t73: f64, t3153: f64, t1043: f64, t106719: f64, t106730: f64, t1089: f64, t1096: f64, t19403: f64, t20195: f64, t25605: f64, t25658: f64, t27595: f64, t27640: f64, t27661: f64, t27664: f64, t27669: f64, t29731: f64, t29739: f64, t29743: f64, t29751: f64, t29759: f64, t4983: f64, t4998: f64, t6245: f64, t6351: f64, t7140: f64, t7159: f64, t93436: f64, t93438: f64, t93502: f64, t93890: f64, t93968: f64, t94023: f64, t94063: f64, t94122: f64, t99953: f64) -> (f64, f64, f64) {
    let t106745 = t355 * t4910;
    let t106763 = t1976 * t6299;
    let t106764 = t106763 * t73;
    let t106768 = t106763 * t3153;
    let t106786 = 0.26341796731742046394e1_f64 * t7140 * t20195 + 0.34694512752820797848e1_f64 * t93436 * t29739 * t106719 - 0.52041769129231196772e1_f64 * t94122 * t29743 * t106745 + 0.13170898365871023197e1_f64 * t25658 * t6351 + 0.17347256376410398924e1_f64 * t93890 * t29739 * t106730 + 0.34694512752820797848e1_f64 * t93502 * t29743 * t93438 + 0.10408353825846239354e2_f64 * t7159 * t93968 * t29751 * t1096 + 0.13170898365871023197e1_f64 * t94023 * t6245 + 0.8673628188205199462e0_f64 * t25605 * t106764 * t27664 - 0.8673628188205199462e0_f64 * t27669 * t106768 * t4983 + 0.4336814094102599731e0_f64 * t27640 * t106768 * t4998 - 0.17347256376410398924e1_f64 * t94063 * t29759 * t106730 + 0.8673628188205199462e0_f64 * t25605 * t29731 * t1043 * t1089 + 0.34694512752820797848e1_f64 * t27661 * t27595 - 0.26341796731742046394e1_f64 * t99953 * t19403;
    (t106745, t106764, t106786)
}
