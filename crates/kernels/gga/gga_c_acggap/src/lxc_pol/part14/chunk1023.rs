//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1023/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1023<F: Float>(t1165: F, t23688: F, t604: F, t7346: F, t7310: F, t8771: F, t1072: F, t31137: F, t513: F, t721: F, t2019: F, t2029: F, t8807: F) -> (F, F, F, F) {
    let t36030 = t7346 * t1165 * t604 * t23688;
    let t36031 = F::cast_from(0.31448092289604152068e-3_f64) * t36030;
    let t36032 = t7310 * t8771;
    let t36036 = t31137 * t1072 * t513 * t721;
    let t36039 = t2019 * t2029 * t8807;
    (t36031, t36032, t36036, t36039)
}
