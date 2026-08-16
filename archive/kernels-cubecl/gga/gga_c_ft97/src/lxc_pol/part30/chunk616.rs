//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 616/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk616<F: Float>(t193: F, t27946: F, t27882: F, t6009: F, t1131: F, t771: F, t6008: F, t263: F, t3821: F, t6753: F, t681: F, t6930: F, t766: F) -> (F, F, F, F, F, F, F, F) {
    let t27947 = t193 * t27946;
    let t27952 = t27882 * t6009;
    let t27953 = t193 * t27952;
    let t27956 = t771 * t1131;
    let t27957 = t6008 * t27956;
    let t27958 = t193 * t27957;
    let t27963 = t263 * t3821;
    let t27964 = t6008 * t27963;
    let t27965 = t193 * t27964;
    let t27968 = t681 * t6753;
    let t27971 = t6930 * t766;
    (t27947, t27953, t27956, t27958, t27963, t27965, t27968, t27971)
}
