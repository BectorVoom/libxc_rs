//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1165/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1165<F: Float>(t34366: F, t5727: F, t1036: F, t11311: F, t13738: F, t5856: F, t11496: F, t185: F, t9386: F, t11435: F, t129: F, t21778: F) -> (F, F, F, F) {
    let t34390 = t34366 * t5727;
    let t34394 = t5856 * t11311 * t1036 * t13738;
    let t34397 = t185 * t9386 * t11496;
    let t34400 = t21778 * t129 * t11435;
    (t34390, t34394, t34397, t34400)
}
