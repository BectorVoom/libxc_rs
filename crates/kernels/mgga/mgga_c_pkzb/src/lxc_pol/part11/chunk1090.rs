//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1090/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1090<F: Float>(t2410: F, t344: F, t148: F, t931: F, t179: F, t404: F, t824: F, t2411: F, t465: F, t154: F, t385: F, t386: F, t4932: F) -> (F, F, F, F, F) {
    let t19140 = F::new(1.0) / t2410 / t344;
    let t19150 = t148 * t931;
    let t19153 = t404 * t179 * t19150 * t824;
    let t19155 = t465 * t2411;
    let t19163 = F::new(5.0) / F::new(486.0) * t385 * t154 * t4932 * t386;
    (t19140, t19150, t19153, t19155, t19163)
}
