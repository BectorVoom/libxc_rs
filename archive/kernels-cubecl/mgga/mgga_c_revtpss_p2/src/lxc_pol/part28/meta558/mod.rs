//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2013;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta558<F: Float>(t2487: F, t93034: F, t10832: F, t25245: F, t25266: F, t2648: F, t2681: F, t7036: F, t820: F, t839: F, t25260: F, t843: F, t2726: F, t10841: F, t10867: F, t64: F, t7043: F, t857: F, t25222: F, t2656: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t93035, t93043, t93045, t93048, t93049, t93054) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2013::<F>(t2487, t93034, t10832, t25245, t25266, t2648, t2681, t7036, t820, t839, t25260, t843);
        let (t93055, t93058, t93060, t93066, t93067, t93069) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2014::<F>(t2726, t93054, t10841, t25245, t10867, t64, t2681, t7043, t820, t857, t25222, t2656);
    (t93035, t93043, t93045, t93048, t93049, t93055, t93058, t93060, t93066, t93067, t93069)
}
