//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 498/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk498<F: Float>(t123: F, t250: F, t132: F, t721: F, t759: F, t762: F, t791: F, t256: F, t729: F, t257: F, t2671: F, t2674: F, t2677: F, t2679: F, t2683: F, t2685: F, t2687: F, t2690: F) -> (F, F, F, F, F, F) {
    let t2723 = t123 * t250;
    let t2736 = t721 * t132 * t759 * t762;
    let t2737 = F::new(0.10685e0) * t2736;
    let t2738 = t132 * t791;
    let t2742 = t729 * t256;
    let t2743 = t2742 * t257;
    let t2754 = -F::new(0.47063e1) * t2671 + F::new(0.31375333333333333334e1) * t2674 - F::new(0.36604555555555555556e1) * t2677 - F::new(0.16068111111111111111e1) * t2679 + F::new(0.28051666666666666666e0) * t2683 - F::new(0.56103333333333333332e0) * t2685 - F::new(0.6545388888888888889e0) * t2687 - F::new(0.46308888888888888888e0) * t2690;
    (t2723, t2737, t2738, t2742, t2743, t2754)
}
