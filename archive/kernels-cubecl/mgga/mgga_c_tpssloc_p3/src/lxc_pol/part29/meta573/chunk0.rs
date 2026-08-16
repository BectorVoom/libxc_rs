//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1990/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1990<F: Float>(t1307: F, t5356: F, t1388: F, t1351: F, t5187: F, t19735: F, t1352: F, t5286: F, t1799: F, t3698: F, t4303: F, t776: F) -> (F, F, F, F, F, F, F) {
    let t56198 = t1307 * t5356;
    let t56404 = t1388 * t5356;
    let t56805 = t5187 * t1351;
    let t57554 = t19735 * t1351;
    let t57643 = t1352 * t5286;
    let t57802 = t1799 * t3698;
    let t57893 = t776 * t4303;
    (t56198, t56404, t56805, t57554, t57643, t57802, t57893)
}
