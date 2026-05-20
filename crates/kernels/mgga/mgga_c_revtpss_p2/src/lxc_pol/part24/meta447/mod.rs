//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1408;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta447<F: Float>(t13665: F, t9863: F, t9866: F, t9575: F, t9572: F, t3863: F, t5569: F, t3860: F, t5571: F, t9419: F, t1882: F, t4010: F, t1885: F, t46722: F, t1389: F, t46856: F, t543: F, t685: F, t72: F, t13955: F, t46946: F, t47198: F, t5665: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48304, t48306, t48313, t48324, t48331, t48333, t48335, t48455) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1408::<F>(t13665, t9863, t9866, t9575, t9572, t3863, t5569, t3860, t5571, t9419, t1882, t4010);
        let (t48518, t48563, t48600, t48792) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1409::<F>(t1885, t46722, t1389, t1882, t46856, t543, t685, t72, t13955, t46946, t47198, t5665);
    (t48304, t48306, t48313, t48324, t48331, t48333, t48335, t48455, t48518, t48563, t48600, t48792)
}
