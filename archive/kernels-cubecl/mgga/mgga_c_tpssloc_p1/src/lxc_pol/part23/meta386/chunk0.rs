//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1190/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1190<F: Float>(t12344: F, t5234: F, t1831: F, t40059: F, t12282: F, t12290: F, t12384: F, t1827: F, t40123: F, t1788: F, t9212: F, t9214: F) -> (F, F, F, F, F, F, F, F) {
    let t53880 = t5234 * t12344;
    let t53901 = t40059 * t1831;
    let t53945 = t5234 * t12282;
    let t54020 = t5234 * t12290;
    let t54042 = t5234 * t12384;
    let t54151 = t40123 * t1827;
    let t54312 = t9212 * t1788;
    let t54314 = t9214 * t1788;
    (t53880, t53901, t53945, t54020, t54042, t54151, t54312, t54314)
}
