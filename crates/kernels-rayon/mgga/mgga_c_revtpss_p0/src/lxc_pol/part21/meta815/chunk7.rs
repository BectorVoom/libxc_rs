//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2992/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2992(t11629: f64, t53703: f64, t3316: f64, t4746: f64, t4891: f64, t16381: f64, t3090: f64, t11620: f64, t11634: f64, t11639: f64, t11663: f64, t11672: f64, t11680: f64, t11877: f64, t15601: f64, t15618: f64, t15707: f64, t15758: f64, t15970: f64, t16210: f64, t19738: f64, t3097: f64, t3117: f64, t3188: f64, t357: f64, t42571: f64, t4825: f64, t4893: f64, t4899: f64) -> f64 {
    let t54564 = t53703 * t11629;
    let t54570 = t4746 * t3316 * t4891;
    let t54578 = t16381 * t3090;
    let t54589 = 0.85748036236139473944e-3_f64 * t15707 * t11639 + 0.45732285992607719436e-2_f64 * t42571 * t4825 + 0.12862205435420921092e-2_f64 * t54564 * t11634 + 0.19055119163586549765e-2_f64 * t3188 * t16210 + 0.64311027177104605458e-3_f64 * t54570 * t11877 - 0.21437009059034868486e-3_f64 * t4899 * t3117 * t4893 * t11620 * t357 + 0.85748036236139473944e-3_f64 * t54578 * t3097 + 0.42874018118069736972e-3_f64 * t15618 * t11680 + 0.85748036236139473944e-3_f64 * t19738 * t11663 - 0.22866142996303859718e-2_f64 * t11672 * t15601 + 0.85748036236139473944e-3_f64 * t15758 * t15970;
    t54589
}
