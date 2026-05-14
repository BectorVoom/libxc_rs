//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1293/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1293<F: Float>(t1359: F, t17347: F, t28: F, t586: F, t5890: F, t119175: F, t27: F, t526: F, t89: F, t105578: F, t105579: F, t16666: F, t16671: F, t95293: F, t105905: F, t16675: F) -> (F, F, F, F, F) {
    let t120086 = t5890 * t28 * t586 * t1359 * t17347;
    let t120090 = t89 * t27 * t526 * t119175;
    let t120093 = t105578 * t105579 * t16666;
    let t120096 = t105578 * t95293 * t16671;
    let t120099 = t105578 * t105905 * t16675;
    (t120086, t120090, t120093, t120096, t120099)
}
