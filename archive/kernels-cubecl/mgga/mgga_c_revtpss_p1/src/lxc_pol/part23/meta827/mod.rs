//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta827 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2683;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta827<F: Float>(t6244: F, t905: F, t11774: F, t4782: F, t53391: F, t1011: F, t15993: F, t18909: F, t11933: F, t19976: F, t3115: F, t42793: F, t6272: F, t11922: F, t16081: F, t19749: F, t20020: F, t3211: F, t15656: F, t4845: F, t19675: F, t372: F, t11947: F, t20016: F, t11875: F, t19757: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t66966, t66972, t66981, t67006, t67015) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2683::<F>(t6244, t905, t11774, t4782, t53391, t1011, t15993, t18909, t11933, t19976, t3115, t42793, t6272);
        let (t67025, t67044, t67048, t67052, t67072, t67152) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2684::<F>(t11922, t16081, t19749, t20020, t3211, t15656, t4845, t19675, t372, t11947, t20016, t11875, t19757);
    (t66966, t66972, t66981, t67006, t67015, t67025, t67044, t67048, t67052, t67072, t67152)
}
