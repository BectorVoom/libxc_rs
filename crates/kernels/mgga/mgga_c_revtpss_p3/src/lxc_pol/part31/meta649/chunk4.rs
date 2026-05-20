//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2143/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2143<F: Float>(t355: F, t4910: F, t1976: F, t6299: F, t73: F, t3153: F, t1043: F, t106719: F, t106730: F, t1089: F, t1096: F, t19403: F, t20195: F, t25605: F, t25658: F, t27595: F, t27640: F, t27661: F, t27664: F, t27669: F, t29731: F, t29739: F, t29743: F, t29751: F, t29759: F, t4983: F, t4998: F, t6245: F, t6351: F, t7140: F, t7159: F, t93436: F, t93438: F, t93502: F, t93890: F, t93968: F, t94023: F, t94063: F, t94122: F, t99953: F) -> (F, F, F) {
    let t106745 = t355 * t4910;
    let t106763 = t1976 * t6299;
    let t106764 = t106763 * t73;
    let t106768 = t106763 * t3153;
    let t106786 = F::cast_from(0.26341796731742046394e1_f64) * t7140 * t20195 + F::cast_from(0.34694512752820797848e1_f64) * t93436 * t29739 * t106719 - F::cast_from(0.52041769129231196772e1_f64) * t94122 * t29743 * t106745 + F::cast_from(0.13170898365871023197e1_f64) * t25658 * t6351 + F::cast_from(0.17347256376410398924e1_f64) * t93890 * t29739 * t106730 + F::cast_from(0.34694512752820797848e1_f64) * t93502 * t29743 * t93438 + F::cast_from(0.10408353825846239354e2_f64) * t7159 * t93968 * t29751 * t1096 + F::cast_from(0.13170898365871023197e1_f64) * t94023 * t6245 + F::cast_from(0.8673628188205199462e0_f64) * t25605 * t106764 * t27664 - F::cast_from(0.8673628188205199462e0_f64) * t27669 * t106768 * t4983 + F::cast_from(0.4336814094102599731e0_f64) * t27640 * t106768 * t4998 - F::cast_from(0.17347256376410398924e1_f64) * t94063 * t29759 * t106730 + F::cast_from(0.8673628188205199462e0_f64) * t25605 * t29731 * t1043 * t1089 + F::cast_from(0.34694512752820797848e1_f64) * t27661 * t27595 - F::cast_from(0.26341796731742046394e1_f64) * t99953 * t19403;
    (t106745, t106764, t106786)
}
