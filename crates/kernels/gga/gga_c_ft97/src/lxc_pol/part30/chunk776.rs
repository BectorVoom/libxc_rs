//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 776/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk776<F: Float>(t35853: F, t852: F, t193: F, t6308: F, t33819: F, t33846: F, t35822: F, t35826: F, t35831: F, t35836: F, t35840: F, t35844: F, t35848: F, t35851: F, t1091: F, t2665: F, t33868: F) -> (F, F, F, F) {
    let t35854 = t852 * t35853;
    let t35856 = t6308 * t193 * t35854;
    let t35858 = t35822 / 2.0 + t33819 + 2.0 / 9.0 * t35826 + 4.0 / 3.0 * t35831 - 2.0 / 3.0 * t35836 - t35840 / 6.0 - t33846 - t35844 / 9.0 - t35848 + 2.0 / 3.0 * t35851 + t35856 / 12.0;
    let t35860 = t2665 * t33868 * t1091;
    (t35854, t35856, t35858, t35860)
}
