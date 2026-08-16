//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2932/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2932<F: Float>(t17579: F, t225: F, t18048: F, t210: F, t974: F, t2985: F, t1597: F, t976: F, t17826: F, t2960: F, t12652: F, t4337: F) -> (F, F, F, F, F, F, F) {
    let t61058 = t17579 * t225;
    let t61061 = t18048 * t225;
    let t61064 = t210 * t974;
    let t61065 = t2985 * t61064;
    let t61066 = t976 * t1597;
    let t61074 = t2960 * t17826;
    let t61078 = t4337 * t12652;
    (t61058, t61061, t61064, t61065, t61066, t61074, t61078)
}
