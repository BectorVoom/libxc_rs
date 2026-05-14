//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1068/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1068<F: Float>(t1410: F, t157: F, t556: F, t1614: F, t9062: F, t2131: F, t2147: F, t309: F, t9793: F, t2146: F, t2342: F, t31978: F, t33566: F, t33635: F, t33648: F, t33656: F, t33662: F, t33672: F, t33681: F, t38662: F, t463: F, t609: F, t6557: F, t7931: F, t7932: F, t9034: F, t9767: F) -> (F,) {
    let t40675 = t556 * t1410 * t157;
    let t40681 = t9062 * t1614;
    let t40691 = t2131 * t2147 * t9793 * t309;
    let t40695 = 0.69389025505641595696e1 * t33635 - 0.13170898365871023197e1 * t33648 + 0.8673628188205199462e0 * t2146 * t2147 * t609 * t6557 - 0.17347256376410398924e1 * t7931 * t7932 * t40675 - 0.8673628188205199462e0 * t31978 + 0.13170898365871023197e1 * t33656 + 0.13170898365871023197e1 * t40681 + t33662 + 0.17347256376410398924e1 * t33566 * t2342 + 0.8673628188205199462e0 * t2146 * t2147 * t9767 * t463 + 0.17347256376410398924e1 * t40691 + t33672 + t33681 - 0.17347256376410398924e1 * t38662 * t9034;
    (t40695,)
}
