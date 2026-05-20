//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2022;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta573<F: Float>(t93174: F, t93371: F, t25410: F, t93341: F, t25413: F, t25374: F, t93169: F, t93191: F, t2439: F, t7048: F, t780: F, t785: F, t25310: F, t25331: F, t25412: F, t93329: F, t25411: F, t25431: F, t2435: F, t25339: F, t11064: F, t7086: F, t1113: F, t2411: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93372, t93374, t93375, t93377, t93378, t93382) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2022::<F>(t93174, t93371, t25410, t93341, t25413, t25374, t93169, t93191, t2439, t7048, t780, t785);
        let (t93384, t93387, t93389, t93391, t93404, t94245) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2023::<F>(t25310, t25331, t25412, t93329, t25411, t25431, t2435, t25339, t11064, t7086, t1113, t2411);
    (t93372, t93374, t93375, t93377, t93378, t93382, t93384, t93387, t93389, t93391, t93404, t94245)
}
