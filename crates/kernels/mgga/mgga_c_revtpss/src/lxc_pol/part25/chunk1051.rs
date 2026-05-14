//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1051/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1051<F: Float>(t1976: F, t3059: F, t7145: F, t1000: F, t1097: F, t1978: F, t25640: F, t25648: F, t25651: F, t25658: F, t25662: F, t25671: F, t25674: F, t25678: F, t25683: F, t25687: F, t25692: F, t25695: F, t25699: F, t3043: F, t3060: F, t3067: F, t3076: F, t3271: F, t3326: F, t342: F, t7102: F, t7137: F, t7140: F, t7144: F, t7156: F, t7167: F, t7170: F, t7174: F, t989: F) -> (F, F, F) {
    let t25700 = t1976 * t3059;
    let t25701 = t7145 * t25700;
    let t25704 = -0.8673628188205199462e0 * t25640 * t7170 - 0.65854491829355115987e0 * t7102 * t3076 - 0.65854491829355115987e0 * t7140 * t3326 - 0.17347256376410398924e1 * t7144 * t25648 + 0.13170898365871023197e1 * t25651 * t3060 + 0.65854491829355115987e0 * t3043 * t1978 + 0.13170898365871023197e1 * t989 * t7137 - 0.13170898365871023197e1 * t25658 * t1097 + 0.65854491829355115987e0 * t342 * t25662 + 0.13170898365871023197e1 * t7102 * t3067 + 0.13170898365871023197e1 * t7140 * t3271 - 0.8673628188205199462e0 * t25671 * t25674 + 0.4336814094102599731e0 * t25671 * t25678 - 0.8673628188205199462e0 * t7167 * t25683 - 0.4336814094102599731e0 * t7167 * t25687 - 0.8673628188205199462e0 * t7156 * t7174 - 0.13170898365871023197e1 * t25692 * t1000 - 0.13170898365871023197e1 * t25695 * t1000 - 0.26020884564615598386e1 * t25699 * t25701;
    (t25700, t25701, t25704)
}
