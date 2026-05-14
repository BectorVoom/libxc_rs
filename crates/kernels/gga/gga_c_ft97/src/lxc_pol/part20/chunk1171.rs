//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1171/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1171<F: Float>(t1882: F, t28201: F, t28110: F, t28167: F, t1443: F, t9952: F, t10051: F, t108152: F, t1091: F, t14071: F, t14076: F, t14082: F, t1901: F, t24429: F, t24696: F, t2574: F, t2606: F, t2619: F, t265: F, t28299: F, t28404: F, t3842: F, t3864: F, t3977: F, t446: F, t6187: F, t6852: F, t729: F, t97522: F, t97817: F, t98061: F, t98063: F, t98065: F) -> (F,) {
    let t111443 = 2.0 / 9.0 * t1882 * t28201;
    let t111452 = 2.0 / 9.0 * t1882 * t28110;
    let t111466 = 2.0 / 9.0 * t1882 * t28167;
    let t111478 = t9952 * t1443;
    let t111486 = t111443 + 2.0 / 27.0 * t1901 * t97522 * t14071 + 2.0 / 3.0 * t446 * t2574 * t265 * t108152 - t111452 + t446 * t729 * t3977 * t24696 / 3.0 + 2.0 / 3.0 * t446 * t2574 * t2619 * t6852 + t1901 * t2606 * t97817 * t1091 / 9.0 + t111466 - 8.0 / 81.0 * t98061 + 2.0 / 81.0 * t98063 + t98065 / 27.0 - 4.0 * t1901 * t28299 * t10051 * t6187 * t3864 - 2.0 / 27.0 * t1901 * t28404 * t14076 - 10.0 / 81.0 * t1901 * t111478 * t14082 + 2.0 / 3.0 * t446 * t729 * t24429 * t3842;
    (t111486,)
}
