//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1147/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1147<F: Float>(t116289: F, t22953: F, t5674: F, t16169: F, t23031: F, t25878: F, t116254: F, t116258: F, t116262: F, t116266: F, t116270: F, t116275: F, t116279: F, t116283: F, t116287: F, t1871: F, t22952: F, t25888: F, t25899: F) -> (F, F, F, F, F) {
    let t116291 = t5674 * t22953 * t116289;
    let t116292 = t23031 * t16169;
    let t116294 = t25878 * t22953 * t116292;
    let t116296 = t116254 / 8.0 - t116258 / 3.0 - 2.0 / 3.0 * t116262 - t116266 / 6.0 - t116270 / 6.0 + t116275 / 6.0 - t116279 / 6.0 - t116283 / 3.0 + 2.0 / 3.0 * t116287 + t116291 - 4.0 / 3.0 * t116294;
    let t116299 = t22952 * t1871 * t25899 * t25888;
    (t116291, t116292, t116294, t116296, t116299)
}
