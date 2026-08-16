//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2144/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2144<F: Float>(t106655: F, t7150: F, t1668: F, t7810: F, t73: F, t1043: F, t106745: F, t106764: F, t1089: F, t1695: F, t19421: F, t25611: F, t25629: F, t25640: F, t25651: F, t25692: F, t27415: F, t27606: F, t27621: F, t27652: F, t27687: F, t29739: F, t29748: F, t29812: F, t29822: F, t29830: F, t29875: F, t29883: F, t4866: F, t4976: F, t6251: F, t7151: F, t7153: F, t7156: F, t7160: F, t7174: F, t7821: F, t93502: F) -> (F, F, F) {
    let t106787 = t7150 * t106655;
    let t106823 = t7810 * t1668;
    let t106824 = t106823 * t73;
    let t106834 = F::cast_from(0.8673628188205199462e0_f64) * t106787 * t7153 - F::cast_from(0.13170898365871023197e1_f64) * t25651 * t19421 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t7821 * t4866 * t1089 + F::cast_from(0.8673628188205199462e0_f64) * t25611 * t29883 * t1043 * t1089 + F::cast_from(0.34694512752820797848e1_f64) * t93502 * t29739 * t106745 - F::cast_from(0.17347256376410398924e1_f64) * t27415 * t29748 + F::cast_from(0.13170898365871023197e1_f64) * t25692 * t6251 - F::cast_from(0.4336814094102599731e0_f64) * t29812 * t7174 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t27687 * t1695 - F::cast_from(0.8673628188205199462e0_f64) * t25629 * t106764 * t27652 + F::cast_from(0.8673628188205199462e0_f64) * t25611 * t106764 * t4976 - F::cast_from(0.8673628188205199462e0_f64) * t25629 * t29875 * t1043 * t1089 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t106824 * t27652 - F::cast_from(0.4336814094102599731e0_f64) * t7156 * t29830 - F::cast_from(0.8673628188205199462e0_f64) * t27621 * t27606 - F::cast_from(0.8673628188205199462e0_f64) * t25640 * t29822;
    (t106823, t106824, t106834)
}
