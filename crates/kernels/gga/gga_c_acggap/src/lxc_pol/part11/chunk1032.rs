//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1032/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1032<F: Float>(t1165: F, t21099: F, t604: F, t7337: F, t23688: F, t7346: F, t7310: F, t8771: F, t1072: F, t31137: F, t513: F, t721: F, t2019: F, t2029: F, t8807: F, t31142: F, t8810: F) -> (F, F, F, F, F, F) {
    let t36026 = t7337 * t1165 * t604 * t21099;
    let t36030 = t7346 * t1165 * t604 * t23688;
    let t36031 = 0.31448092289604152068e-3 * t36030;
    let t36032 = t7310 * t8771;
    let t36036 = t31137 * t1072 * t513 * t721;
    let t36039 = t2019 * t2029 * t8807;
    let t36040 = 7.0 / 24.0 * t36039;
    let t36041 = t31142 * t8810;
    (t36026, t36031, t36032, t36036, t36040, t36041)
}
