//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1157/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1157<F: Float>(t16150: F, t92196: F, t25928: F, t5674: F, t16011: F, t23031: F, t22953: F, t116425: F, t116272: F, t22952: F, t22986: F, t23054: F, t29665: F, t1871: F, t4495: F, t473: F, t5675: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116446 = t92196 * t16150;
    let t116448 = t5674 * t25928 * t116446;
    let t116451 = t23031 * t16011;
    let t116453 = t5674 * t22953 * t116451;
    let t116456 = t5674 * t25928 * t116425;
    let t116460 = t22952 * t25928 * t22986 * t116272;
    let t116462 = t23054 * t29665;
    let t116463 = t116462 / 27.0;
    let t116467 = t22952 * t1871 * t5675 * t4495 * t473;
    (t116446, t116448, t116451, t116453, t116456, t116460, t116462, t116463, t116467)
}
