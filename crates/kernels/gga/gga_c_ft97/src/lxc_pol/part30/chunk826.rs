//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 826/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk826<F: Float>(t24223: F, t7437: F, t1403: F, t33278: F, t681: F, t33253: F, t683: F, t33244: F, t2371: F, t33452: F, t24178: F, t2567: F, t7536: F, t33792: F, t33268: F, t33531: F, t761: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t140585 = t7437 * t24223;
    let t140588 = t1403 * t681 * t33278;
    let t140594 = t683 * t33253;
    let t140605 = t1403 * t681 * t33244;
    let t140627 = t2371 * t33452;
    let t140649 = t7437 * t24178;
    let t140653 = t7536 * t2567;
    let t140664 = t1403 * t681 * t33792;
    let t140684 = t1403 * t681 * t33268;
    let t140686 = t33531 * t761;
    (t140585, t140588, t140594, t140605, t140627, t140649, t140653, t140664, t140684, t140686)
}
