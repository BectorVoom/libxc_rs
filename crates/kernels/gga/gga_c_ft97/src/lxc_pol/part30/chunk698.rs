//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 698/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk698<F: Float>(t258: F, t7484: F, t684: F, t2599: F, t6088: F, t6154: F, t729: F, t2469: F, t7502: F, t1449: F, t6061: F, t762: F, t713: F, t7560: F, t265: F, t33452: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33715 = t258 * t7484;
    let t33716 = t33715 * t684;
    let t33717 = t2599 * t33716;
    let t33721 = t729 * t6154 * t6088;
    let t33725 = t729 * t2469 * t7502;
    let t33728 = t6061 * t1449;
    let t33730 = t729 * t762 * t33728;
    let t33734 = t729 * t7560 * t713;
    let t33738 = t729 * t265 * t33452;
    (t33715, t33716, t33717, t33721, t33725, t33728, t33730, t33734, t33738)
}
