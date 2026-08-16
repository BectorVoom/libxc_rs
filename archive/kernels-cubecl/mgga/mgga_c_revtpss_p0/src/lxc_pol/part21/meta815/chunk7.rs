//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2992/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2992<F: Float>(t11629: F, t53703: F, t3316: F, t4746: F, t4891: F, t16381: F, t3090: F, t11620: F, t11634: F, t11639: F, t11663: F, t11672: F, t11680: F, t11877: F, t15601: F, t15618: F, t15707: F, t15758: F, t15970: F, t16210: F, t19738: F, t3097: F, t3117: F, t3188: F, t357: F, t42571: F, t4825: F, t4893: F, t4899: F) -> F {
    let t54564 = t53703 * t11629;
    let t54570 = t4746 * t3316 * t4891;
    let t54578 = t16381 * t3090;
    let t54589 = F::cast_from(0.85748036236139473944e-3_f64) * t15707 * t11639 + F::cast_from(0.45732285992607719436e-2_f64) * t42571 * t4825 + F::cast_from(0.12862205435420921092e-2_f64) * t54564 * t11634 + F::cast_from(0.19055119163586549765e-2_f64) * t3188 * t16210 + F::cast_from(0.64311027177104605458e-3_f64) * t54570 * t11877 - F::cast_from(0.21437009059034868486e-3_f64) * t4899 * t3117 * t4893 * t11620 * t357 + F::cast_from(0.85748036236139473944e-3_f64) * t54578 * t3097 + F::cast_from(0.42874018118069736972e-3_f64) * t15618 * t11680 + F::cast_from(0.85748036236139473944e-3_f64) * t19738 * t11663 - F::cast_from(0.22866142996303859718e-2_f64) * t11672 * t15601 + F::cast_from(0.85748036236139473944e-3_f64) * t15758 * t15970;
    t54589
}
