//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 880/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk880<F: Float>(t2212: F, t23548: F, t13220: F, t1647: F, t5855: F, t2221: F, t1651: F, t1643: F, t9115: F, t1359: F, t604: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23555 = t23548 * t2212;
    let t23556 = t13220 * t23555;
    let t23559 = t5855 * t1647;
    let t23560 = t2221 * t23559;
    let t23563 = t5855 * t1651;
    let t23564 = t2221 * t23563;
    let t23567 = t5855 * t1643;
    let t23568 = t9115 * t23567;
    let t23571 = t604 * t1359;
    (t23555, t23556, t23559, t23560, t23563, t23564, t23567, t23568, t23571)
}
