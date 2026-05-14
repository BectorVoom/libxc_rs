//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1288/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1288<F: Float>(t1466: F, t2399: F, t7027: F, t25485: F, t6963: F, t25488: F, t10683: F, t113581: F, t113640: F, t113950: F, t114011: F, t114120: F, t114488: F, t1218: F, t15175: F, t193: F, t25389: F, t25446: F, t28972: F, t312: F, t4162: F, t6210: F, t6216: F, t6217: F, t6223: F) -> (F,) {
    let t115003 = t1466 * t2399 * t7027;
    let t115016 = t6963 * t25485;
    let t115024 = t6963 * t25488 / 9.0;
    let t115028 = -2.0 / 3.0 * t6210 * t28972 + 2.0 / 27.0 * t115003 - 2.0 * t114120 + 2.0 * t114488 * t312 + 2.0 * t6216 * t10683 * t25446 * t4162 + 2.0 * t6216 * t10683 * t6217 * t15175 + 2.0 / 27.0 * t115016 + 4.0 * t113950 - 2.0 / 3.0 * t1466 * t193 * t113581 * t6223 - t115024 - t1218 * t25389 - 2.0 * t113640 + 4.0 * t114011;
    (t115028,)
}
