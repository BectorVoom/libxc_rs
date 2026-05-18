//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 776/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk776<F: Float>(t7584: F, t856: F, t7641: F, t33811: F, t7512: F, t33288: F, t7638: F, t7642: F, t6307: F, t631: F) -> (F, F, F, F, F, F) {
    let t33812 = t7584 * t856;
    let t33813 = t7641 * t33812;
    let t33815 = t33811 * t7512 * t33813;
    let t33818 = t7638 * t33288 * t7642;
    let t33819 = F::new(2.0) / F::new(9.0) * t33818;
    let t33820 = t6307 * t631;
    (t33812, t33813, t33815, t33818, t33819, t33820)
}
