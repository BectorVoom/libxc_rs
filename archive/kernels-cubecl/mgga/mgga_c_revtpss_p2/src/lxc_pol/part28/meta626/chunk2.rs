//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2237/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2237<F: Float>(t15654: F, t1976: F, t1000: F, t100403: F, t1096: F, t15648: F, t225: F, t25464: F, t25473: F, t25476: F, t25597: F, t25629: F, t25658: F, t25695: F, t27419: F, t27426: F, t27441: F, t27550: F, t27595: F, t27651: F, t27652: F, t27695: F, t27699: F, t3042: F, t3067: F, t3271: F, t342: F, t385: F, t4772: F, t4773: F, t4947: F, t4975: F, t7135: F, t7145: F, t7151: F, t7159: F, t7822: F, t93429: F, t93498: F, t94016: F, t99762: F) -> F {
    let t100760 = t15654 * t1976;
    let t100794 = F::cast_from(0.13170898365871023197e1_f64) * t27699 * t3271 + F::cast_from(0.13170898365871023197e1_f64) * t27550 * t3067 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t99762 * t27652 - F::cast_from(0.8673628188205199462e0_f64) * t25629 * t27651 * t4975 * t3042 - F::cast_from(0.13170898365871023197e1_f64) * t100760 * t1000 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t100403 * t225 * t385 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7145 * t1976 * t15648 - F::cast_from(0.13170898365871023197e1_f64) * t25695 * t4773 + F::cast_from(0.26341796731742046394e1_f64) * t25658 * t4947 - F::cast_from(0.34694512752820797848e1_f64) * t27419 * t25597 + F::cast_from(0.17347256376410398924e1_f64) * t25473 * t27441 - F::cast_from(0.52041769129231196772e1_f64) * t7159 * t25464 * t27426 * t1096 - F::cast_from(0.52041769129231196772e1_f64) * t94016 * t27695 * t93498 + F::cast_from(0.34694512752820797848e1_f64) * t25476 * t27595 + F::cast_from(0.17347256376410398924e1_f64) * t93429 * t7822 + F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7145 * t7135 * t4772;
    t100794
}
