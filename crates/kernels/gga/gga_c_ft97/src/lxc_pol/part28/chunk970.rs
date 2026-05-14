//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 970/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk970<F: Float>(t136847: F, t138894: F, t138899: F, t138924: F, t138927: F, t138930: F, t139009: F, t139121: F, t147412: F, t147416: F, t147425: F, t147429: F, t147432: F, t147435: F, t147440: F, t147445: F, t2001: F, t23847: F, t23866: F, t32774: F, t3380: F, t3392: F, t34857: F, t34910: F, t48841: F, t7318: F, t8812: F, t8838: F, t8859: F, t935: F) -> (F,) {
    let t147448 = t138894 + 0.24167761770734866964e0 * t138899 - 0.1422705865505209271e0 * t138924 - t138927 - 0.19592980390298668092e-1 * t138930 + 0.64021763947734417195e0 * t32774 * t136847 * t34910 + 0.30552173028732381488e-1 * t2001 * t147412 - 0.15276086514366190744e-1 * t3392 * t147416 + 0.20527106943485609994e0 * t48841 * t34857 + 0.20527106943485609994e0 * t8812 * t7318 * t3380 + 0.41054213886971219988e0 * t8859 * t147425 + 0.10947790369858991997e2 * t23866 * t147429 - 0.45306850413028723348e0 * t23847 * t147432 + 0.45306850413028723348e0 * t8838 * t147435 + 0.35314306798406949389e-2 * t139009 * t935 - 0.45306850413028723348e0 * t23847 * t147440 - 0.58778941170896004276e-1 * t139121 * t147445;
    (t147448,)
}
