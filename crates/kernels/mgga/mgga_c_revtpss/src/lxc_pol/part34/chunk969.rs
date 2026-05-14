//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 969/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk969<F: Float>(t24697: F, t459: F, t225: F, t480: F, t1774: F, t6622: F, t1250: F, t3720: F, t6587: F, t247: F, t3719: F, t12900: F, t17629: F, t21170: F, t21189: F, t21193: F, t21216: F, t21234: F, t21249: F, t24681: F, t24684: F, t3718: F, t484: F, t5381: F, t5384: F, t6683: F) -> (F, F, F, F) {
    let t24698 = t24697 * t459;
    let t24699 = t24698 * t225;
    let t24700 = t24699 * t480;
    let t24704 = t1774 * t6622;
    let t24705 = t24704 * t1250;
    let t24706 = t3720 * t24705;
    let t24713 = t1774 * t6587;
    let t24715 = t247 * t3719 * t24713;
    let t24722 = -0.53100265402527852012e-1 * t24681 * t484 + 0.21722835846488666732e-1 * t24684 * t484 + 0.21437009059034868486e-3 * t24700 * t484 + t21170 / 216.0 - 0.64311027177104605458e-3 * t3718 * t24706 + t12900 + 0.85748036236139473944e-3 * t21189 - 0.85748036236139473944e-3 * t5381 * t6683 - 0.57165357490759649295e-3 * t21193 + 0.12862205435420921092e-2 * t5384 * t24715 - 0.57165357490759649295e-3 * t21216 + t17629 / 432.0 + 0.47637797908966374413e-3 * t21234 + t21249 / 54.0;
    (t24698, t24704, t24713, t24722)
}
