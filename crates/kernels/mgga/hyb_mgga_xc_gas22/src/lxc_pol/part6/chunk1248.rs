//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1248/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1248<F: Float>(t28877: F, t28880: F, t28883: F, t28885: F, t28887: F, t28890: F, t28892: F, t28894: F, t28896: F, t28899: F, t28901: F, t28904: F, t24556: F, t24559: F, t24562: F, t24658: F, t24661: F, t24664: F, t24667: F, t24670: F, t24673: F, t28907: F, t28917: F, t28919: F) -> (F, F) {
    let t29288 = 0.2366859375e0 * t28877 - 0.157790625e0 * t28880 - 0.6618234375e1 * t28883 + 0.264729375e1 * t28885 - 0.3529725e1 * t28887 - 0.3529725e1 * t28890 - 0.17648625e1 * t28892 - 0.157790625e0 * t28894 + 0.6311625e0 * t28896 + 0.6311625e0 * t28899 + 0.31558125e0 * t28901 - 0.6311625e0 * t28904;
    let t29301 = 0.10589175e2 * t28907 + 0.6311625e0 * t28917 + 0.3529725e1 * t28919 - 0.32136222222222222223e1 * t24556 + 0.27545333333333333334e1 * t24559 - 0.103295e1 * t24562 + 0.13892666666666666667e1 * t24658 + 0.13892666666666666667e1 * t24661 - 0.18523555555555555555e1 * t24664 - 0.41678e0 * t24667 - 0.83356e0 * t24670 - 0.41678e0 * t24673;
    (t29288, t29301)
}
