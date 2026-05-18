//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 777/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk777<F: Float>(t10480: F, t4265: F, t2874: F, t2399: F, t865: F, t89: F, t2682: F, t875: F, t2862: F, t871: F, t2413: F, t835: F, t882: F) -> (F, F, F, F, F, F) {
    let t10509 = t4265 * t10480;
    let t10510 = t2874 * t10509;
    let t10514 = t89 * t2399 * t865;
    let t10516 = t2682 * t875;
    let t10518 = t2862 * t871 * t10516;
    let t10522 = t835 * t882 * t2413;
    (t10509, t10510, t10514, t10516, t10518, t10522)
}
