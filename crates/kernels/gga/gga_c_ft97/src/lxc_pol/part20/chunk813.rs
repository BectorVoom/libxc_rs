//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 813/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk813<F: Float>(t218: F, t811: F, t820: F, t25057: F, t1701: F, t2726: F, t6027: F, t444: F, t6240: F, t2691: F) -> (F, F, F, F) {
    let t25058 = t218 * t811;
    let t25059 = t25058 * t820;
    let t25060 = t25057 * t25059;
    let t25064 = t1701 * t6027 * t2726;
    let t25069 = t6240 * t444;
    let t25070 = t2691 * t25069;
    (t25060, t25064, t25069, t25070)
}
