//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2200/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2200<F: Float>(t1976: F, t4743: F, t1695: F, t988: F, t27543: F, t342: F, t1043: F, t1089: F, t1096: F, t1097: F, t15579: F, t16328: F, t1985: F, t25605: F, t25629: F, t25695: F, t25699: F, t27411: F, t27440: F, t27444: F, t27651: F, t27679: F, t27691: F, t3059: F, t3066: F, t4941: F, t4975: F, t5015: F, t7102: F, t7135: F, t7144: F, t7145: F, t7159: F, t7160: F, t7810: F, t7822: F, t93485: F, t93497: F, t93498: F, t93921: F, t94122: F, t999: F) -> F {
    let t99629 = t4743 * t1976;
    let t99638 = t1695 * t988;
    let t99666 = t342 * t27543;
    let t99673 = F::cast_from(0.17347256376410398924e1_f64) * t7159 * t7160 * t7135 * t5015 - F::cast_from(0.26020884564615598386e1_f64) * t25699 * t7145 * t7810 * t3059 + F::cast_from(0.13170898365871023197e1_f64) * t7102 * t16328 - F::cast_from(0.13170898365871023197e1_f64) * t99629 * t1097 + F::cast_from(0.65854491829355115987e0_f64) * t7102 * t15579 - F::cast_from(0.34694512752820797848e1_f64) * t93497 * t27651 * t4975 * t3066 - F::cast_from(0.69389025505641595696e1_f64) * t93921 * t1985 * t99638 * t999 - F::cast_from(0.52041769129231196772e1_f64) * t94122 * t27691 * t93498 + F::cast_from(0.8673628188205199462e0_f64) * t93485 * t7822 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t27444 * t1096 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t27411 * t1043 * t1089 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t27679 * t1043 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t27440 * t1043 * t1089 + F::cast_from(0.13170898365871023197e1_f64) * t25695 * t4941 - F::cast_from(0.13170898365871023197e1_f64) * t99666 * t1097 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t27444 * t1043 * t1089;
    t99673
}
