//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1809;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta495<F: Float>(t1599: F, t6699: F, t1922: F, t4542: F, t1625: F, t6703: F, t6706: F, t7561: F, t986: F, t23365: F, t7565: F, t23336: F, t7553: F, t1955: F, t4693: F, t3174: F, t2775: F, t387: F, t3961: F, t23329: F, t221: F, t4509: F, t1926: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25400, t25403, t25406, t25407, t25410, t25413, t25416) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1809::<F>(t1599, t6699, t1922, t4542, t1625, t6703, t6706, t7561, t986, t23365, t7565, t23336, t7553);
        let (t25420, t25423, t25424, t25425, t25429) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1810::<F>(t1955, t4693, t3174, t2775, t387, t3961, t23329, t221, t4509, t1926);
    (t25400, t25403, t25406, t25407, t25410, t25413, t25416, t25420, t25423, t25424, t25425, t25429)
}
