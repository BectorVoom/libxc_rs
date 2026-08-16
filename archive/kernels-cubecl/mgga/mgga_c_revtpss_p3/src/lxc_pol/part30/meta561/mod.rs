//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2005;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta561<F: Float>(t10836: F, t25227: F, t2661: F, t2482: F, t596: F, t7036: F, t2487: F, t10832: F, t25245: F, t25266: F, t2648: F, t2681: F, t820: F, t839: F, t25260: F, t843: F, t2726: F, t10841: F, t10867: F, t64: F, t7043: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t93031, t93034, t93035, t93043, t93045, t93048) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2005::<F>(t10836, t25227, t2661, t2482, t596, t7036, t2487, t10832, t25245, t25266, t2648, t2681, t820);
        let (t93049, t93055, t93058, t93060, t93066) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2006::<F>(t839, t93048, t25260, t820, t843, t2726, t10841, t25245, t10867, t64, t2681, t7043);
    (t93031, t93034, t93035, t93043, t93045, t93048, t93049, t93055, t93058, t93060, t93066)
}
