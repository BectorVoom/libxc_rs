//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 533/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk533<F: Float>(t373: F, t4772: F, t371: F, t372: F, t225: F, t4746: F, t366: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F) -> (F, F, F, F) {
    let t4852 = t373 * t4772;
    let t4854 = t371 * t372 * t4852;
    let t4857 = t4746 * t225;
    let t4858 = t4857 * t366;
    let t4866 = -t4589 + t4592 + t4594 - t4597 + t4634 + t4638 + t4716 + t4718 - t4721 - t4723 + t4727 - t4731 - t4736;
    (t4854, t4857, t4858, t4866)
}
