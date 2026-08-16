//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 783/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk783<F: Float>(t2665: F, t33868: F, t684: F, t6317: F, t7611: F, t824: F, t2781: F, t1486: F, t193: F, t2035: F, t7590: F, t811: F) -> (F, F, F, F, F, F) {
    let t33870 = t2665 * t33868 * t684;
    let t33871 = t6317 * t33870;
    let t33873 = t7611 * t824;
    let t33874 = t2781 * t33873;
    let t33876 = t1486 * t193 * t33874;
    let t33885 = t2035 * t7590 * t811;
    (t33870, t33871, t33873, t33874, t33876, t33885)
}
