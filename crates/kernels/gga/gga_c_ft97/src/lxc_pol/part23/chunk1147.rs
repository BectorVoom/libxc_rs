//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1147/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1147<F: Float>(t1882: F, t28264: F, t6907: F, t737: F, t28146: F, t8392: F, t38953: F, t6918: F, t28178: F, t2567: F, t6940: F, t28205: F, t28209: F, t24668: F, t256: F, t28330: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t110946 = 4.0 / 9.0 * t1882 * t28264;
    let t110950 = t737 * t6907;
    let t110961 = 4.0 / 9.0 * t8392 * t28146;
    let t110962 = t38953 * t6918;
    let t110988 = 2.0 / 9.0 * t1882 * t28178;
    let t111016 = t2567 * t6940;
    let t111045 = 2.0 / 27.0 * t8392 * t28205;
    let t111047 = 4.0 / 27.0 * t8392 * t28209;
    let t111048 = t256 * t24668;
    let t111068 = 2.0 / 27.0 * t1882 * t28330;
    (t110946, t110950, t110961, t110962, t110988, t111016, t111045, t111047, t111048, t111068)
}
