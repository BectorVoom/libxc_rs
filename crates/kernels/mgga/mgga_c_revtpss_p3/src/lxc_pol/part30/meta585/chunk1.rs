//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2041/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2041<F: Float>(t7282: F, t9646: F, t2022: F, t22: F, t25937: F, t93139: F, t1955: F, t25920: F, t4075: F, t2435: F, t26061: F, t1385: F, t7274: F) -> (F, F, F, F, F) {
    let t94696 = t9646 * t7282;
    let t94698 = t25937 * t2022 * t22;
    let t94700 = F::cast_from(0.43639970290213137151e-3_f64) * t94696 * t94698;
    let t94701 = t93139 * t7282;
    let t94703 = F::cast_from(0.51727911450665971904e-3_f64) * t94701 * t94698;
    let t94705 = t1955 * t25920 * t4075;
    let t94714 = t2435 * t26061;
    let t94716 = t1385 * t7274;
    (t94700, t94703, t94705, t94714, t94716)
}
