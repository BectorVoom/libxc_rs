//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2205/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2205<F: Float>(t988: F, t999: F, t73: F, t99729: F, t1647: F, t7135: F, t1078: F, t1982: F, t3140: F, t4930: F, t1089: F, t1097: F, t15885: F, t1976: F, t25461: F, t25464: F, t25470: F, t25588: F, t25629: F, t25681: F, t25699: F, t27426: F, t27568: F, t27576: F, t27587: F, t27609: F, t27651: F, t27652: F, t3059: F, t3075: F, t3076: F, t3270: F, t4866: F, t4975: F, t7144: F, t7145: F, t7151: F, t7160: F, t7167: F, t7170: F, t7174: F, t7821: F, t7825: F, t7828: F, t7829: F, t93502: F, t94005: F) -> (F, F) {
    let t99858 = t988 * t999;
    let t99877 = t99729 * t73;
    let t99881 = t1647 * t7135;
    let t99886 = t1982 * t4930 * t3140 * t1078;
    let t99901 = F::cast_from(0.52041769129231196772e1_f64) * t25699 * t7160 * t7828 * t3059 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t27426 * t988 - F::cast_from(0.4336814094102599731e0_f64) * t7825 * t25588 + F::cast_from(0.34694512752820797848e1_f64) * t93502 * t27651 * t4975 * t99858 + F::cast_from(0.52041769129231196772e1_f64) * t7151 * t25464 * t7821 * t3270 + F::cast_from(0.8673628188205199462e0_f64) * t94005 * t7829 - F::cast_from(0.65854491829355115987e0_f64) * t27568 * t3076 - F::cast_from(0.8673628188205199462e0_f64) * t27587 * t7174 + F::cast_from(0.17347256376410398924e1_f64) * t27609 * t25470 - F::cast_from(0.34694512752820797848e1_f64) * t25461 * t27576 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t99877 * t27652 - F::cast_from(0.13170898365871023197e1_f64) * t99881 * t1097 - F::cast_from(0.8673628188205199462e0_f64) * t99886 * t7170 - F::cast_from(0.8673628188205199462e0_f64) * t7167 * t25681 * t4866 * t1089 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7145 * t1976 * t15885 - F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7160 * t7828 * t3075;
    (t99877, t99901)
}
