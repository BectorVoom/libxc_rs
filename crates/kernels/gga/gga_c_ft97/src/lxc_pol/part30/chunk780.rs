//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 780/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk780<F: Float>(t10248: F, t33847: F, t684: F, t6317: F, t10570: F, t33830: F, t1486: F, t193: F, t2781: F, t33835: F, t7611: F, t856: F) -> (F, F, F, F, F, F, F) {
    let t33849 = t10248 * t33847 * t684;
    let t33850 = t6317 * t33849;
    let t33852 = t10570 * t33830;
    let t33854 = t1486 * t193 * t33852;
    let t33855 = t2781 * t33835;
    let t33857 = t1486 * t193 * t33855;
    let t33859 = t7611 * t856;
    (t33849, t33850, t33852, t33854, t33855, t33857, t33859)
}
