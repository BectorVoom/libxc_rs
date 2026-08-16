//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1209/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1209(t1410: f64, t157: f64, t556: f64, t1614: f64, t9062: f64, t2131: f64, t2147: f64, t309: f64, t9793: f64, t2146: f64, t2342: f64, t31978: f64, t33566: f64, t33635: f64, t33648: f64, t33656: f64, t33662: f64, t33672: f64, t33681: f64, t38662: f64, t463: f64, t609: f64, t6557: f64, t7931: f64, t7932: f64, t9034: f64, t9767: f64) -> f64 {
    let t40675 = t556 * t1410 * t157;
    let t40681 = t9062 * t1614;
    let t40691 = t2131 * t2147 * t9793 * t309;
    let t40695 = 0.69389025505641595696e1_f64 * t33635 - 0.13170898365871023197e1_f64 * t33648 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t609 * t6557 - 0.17347256376410398924e1_f64 * t7931 * t7932 * t40675 - 0.8673628188205199462e0_f64 * t31978 + 0.13170898365871023197e1_f64 * t33656 + 0.13170898365871023197e1_f64 * t40681 + t33662 + 0.17347256376410398924e1_f64 * t33566 * t2342 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t9767 * t463 + 0.17347256376410398924e1_f64 * t40691 + t33672 + t33681 - 0.17347256376410398924e1_f64 * t38662 * t9034;
    t40695
}
