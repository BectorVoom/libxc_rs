//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1138/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1138<F: Float>(t1403: F, t2399: F, t6843: F, t27957: F, t681: F, t28466: F, t42109: F, t6003: F, t24211: F, t6745: F, t24223: F, t28163: F, t8392: F, t10052: F, t676: F, t28341: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109787 = t1403 * t2399 * t6843;
    let t109793 = 2.0 / 9.0 * t1403 * t681 * t27957;
    let t109798 = 2.0 / 9.0 * t1403 * t681 * t28466;
    let t109799 = t42109 * t6003;
    let t109809 = t6745 * t24211;
    let t109822 = t6745 * t24223 / 9.0;
    let t109844 = 4.0 / 27.0 * t8392 * t28163;
    let t109848 = t676 * t10052;
    let t109863 = 4.0 / 27.0 * t8392 * t28341;
    (t109787, t109793, t109798, t109799, t109809, t109822, t109844, t109848, t109863)
}
