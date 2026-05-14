//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1288/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1288<F: Float>(t1882: F, t31175: F, t18391: F, t6187: F, t10002: F, t111239: F, t111241: F, t111252: F, t111254: F, t111256: F, t111262: F, t111264: F, t111266: F, t111276: F, t111290: F, t111310: F, t242: F, t2568: F, t31244: F, t446: F, t5064: F, t6061: F, t729: F) -> (F, F) {
    let t124924 = t1882 * t31175;
    let t124926 = t18391 * t6187;
    let t124931 = t111239 + t111241 + t111252 + t111254 - 2.0 / 3.0 * t446 * t729 * t10002 * t31244 - 2.0 / 3.0 * t446 * t729 * t2568 * t6061 * t5064 - t111256 - t111262 + 2.0 / 9.0 * t124924 + t111264 + t111266 - t446 * t242 * t124926 / 3.0 + t111276 + 8.0 / 27.0 * t111290 - t111310;
    (t124926, t124931)
}
