//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1328/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1328<F: Float>(t1445: F, t25912: F, t689: F, t7282: F, t9646: F, t2022: F, t22: F, t25937: F, t93139: F, t1955: F, t25920: F, t4075: F) -> (F, F, F, F) {
    let t94694 = t689 * t25912 * t1445;
    let t94696 = t9646 * t7282;
    let t94698 = t25937 * t2022 * t22;
    let t94700 = F::cast_from(0.43639970290213137151e-3_f64) * t94696 * t94698;
    let t94701 = t93139 * t7282;
    let t94703 = F::cast_from(0.51727911450665971904e-3_f64) * t94701 * t94698;
    let t94705 = t1955 * t25920 * t4075;
    (t94694, t94700, t94703, t94705)
}
