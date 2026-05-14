//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 672/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk672<F: Float>(t169: F, t289: F, t4598: F, t274: F, t413: F, t39: F, t745: F, t1452: F, t532: F, t145: F, t242: F, t4867: F, t5700: F, t5703: F, t5707: F, t5710: F, t5713: F, t5717: F, t5718: F, t5723: F, t5726: F) -> (F,) {
    let t5730 = 0.31835665774679373271e-1 * t169 * t289 * t4598;
    let t5732 = 0.12798016258123051272e1 * t413 * t274;
    let t5733 = t39 * t745;
    let t5735 = t532 * t1452;
    let t5739 = t5700 - 0.42447554366239164361e0 * t5703 - t5707 + 0.15917832887339686635e0 * t5710 + 0.3183566577467937327e0 * t5713 + t5717 - 0.31835665774679373271e-1 * t169 * t5718 * t242 - 0.95506997324038119813e-1 * t5723 - 0.95506997324038119813e-1 * t5726 - t5730 - t5732 + 0.9598512193592288454e0 * t5733 - 0.3199504064530762818e0 * t5735 + 0.533250677421793803e-1 * t145 * t4867;
    (t5739,)
}
