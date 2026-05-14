//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1266/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1266<F: Float>(t1882: F, t29253: F, t29317: F, t29107: F, t8392: F, t10447: F, t10666: F, t1476: F, t15073: F, t1901: F, t2413: F, t2874: F, t29150: F, t29259: F, t4146: F, t4151: F, t446: F, t7105: F, t840: F, t871: F, t99034: F, t99186: F, t99197: F, t99199: F, t99219: F, t99229: F, t99260: F, t99271: F) -> (F,) {
    let t114238 = 4.0 / 9.0 * t1882 * t29253;
    let t114244 = 2.0 / 9.0 * t1882 * t29317;
    let t114247 = 2.0 / 27.0 * t8392 * t29107;
    let t114263 = 2.0 / 9.0 * t1901 * t99034 * t4151 + t1901 * t2874 * t29259 * t2413 / 9.0 - t114238 + 2.0 / 9.0 * t1901 * t10447 * t29150 - 2.0 / 27.0 * t99197 - t114244 - 8.0 / 81.0 * t99199 - t114247 - 4.0 / 9.0 * t99219 + t99229 + t446 * t840 * t10666 * t7105 / 3.0 + t446 * t840 * t871 * t1476 * t15073 / 3.0 + 2.0 / 3.0 * t99260 + 2.0 / 9.0 * t1901 * t99186 * t4146 - 8.0 / 27.0 * t99271;
    (t114263,)
}
