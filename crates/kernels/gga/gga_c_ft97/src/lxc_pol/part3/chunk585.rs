//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 585/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk585<F: Float>(t12: F, t52: F, t1593: F, t1609: F, t1620: F, t5544: F, t25: F, t409: F, t1602: F, t29: F, t31: F, t122: F, t170: F, t7239: F, t30: F, t23: F, t2999: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7853 = t52 * t12;
    let t7857 = t1609 * t1593;
    let t7861 = t5544 * t1620;
    let t7876 = t409 * t25;
    let t7877 = t1602 * t7876;
    let t7905 = 1.0 / t31 / t29;
    let t7906 = t122 * t7905;
    let t7911 = 4.0 * t170 * t7239;
    let t7913 = 1.0 / t30 / t7911;
    let t7914 = t25 * t7913;
    let t7943 = t2999 * t23;
    (t7853, t7857, t7861, t7876, t7877, t7906, t7911, t7914, t7943)
}
