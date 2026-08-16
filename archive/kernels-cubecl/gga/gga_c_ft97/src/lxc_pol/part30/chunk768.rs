//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 768/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk768<F: Float>(t684: F, t7502: F, t10007: F, t24789: F, t6075: F, t258: F, t7484: F, t2599: F, t6088: F, t6154: F, t729: F, t2469: F) -> (F, F, F, F, F, F, F, F) {
    let t33708 = t7502 * t684;
    let t33709 = t10007 * t33708;
    let t33712 = t24789 * t6075;
    let t33715 = t258 * t7484;
    let t33716 = t33715 * t684;
    let t33717 = t2599 * t33716;
    let t33721 = t729 * t6154 * t6088;
    let t33725 = t729 * t2469 * t7502;
    (t33708, t33709, t33712, t33715, t33716, t33717, t33721, t33725)
}
