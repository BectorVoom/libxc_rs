//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1206/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1206(t35814: f64, t35816: f64, t35818: f64, t35827: f64, t35829: f64, t35837: f64, t35844: f64, t35848: f64, t35850: f64, t31603: f64, t31605: f64, t35823: f64, t35833: f64, t35835: f64, t35841: f64, t35846: f64, t35853: f64, t35856: f64) -> f64 {
    let t37731 = 0.16006300097412701803e-1_f64 * t35814;
    let t37732 = 0.42874018118069736972e-3_f64 * t35816;
    let t37733 = 0.28582678745379824648e-3_f64 * t35818;
    let t37735 = 0.28582678745379824648e-3_f64 * t35827;
    let t37736 = 0.16006300097412701803e-1_f64 * t35829;
    let t37739 = 0.25724410870841842184e-2_f64 * t35837;
    let t37741 = 0.42874018118069736972e-3_f64 * t35844;
    let t37743 = 0.16809375e0_f64 * t35848;
    let t37744 = 0.1120625e0_f64 * t35850;
    let t37747 = 13.0_f64 / 72.0_f64 * t31603 + 0.76220476654346199063e-2_f64 * t31605 + t37731 + t37732 + t37733 + 0.21437009059034868486e-3_f64 * t35823 + t37735 - t37736 + 0.37737710747524982483e-2_f64 * t35833 - 0.51448821741683684367e-2_f64 * t35835 + t37739 + 0.10718504529517434243e-2_f64 * t35841 + t37741 - 0.34299214494455789578e-1_f64 * t35846 - t37743 - t37744 + 0.4584375e-1_f64 * t35853 + 0.22921875e-1_f64 * t35856;
    t37747
}
