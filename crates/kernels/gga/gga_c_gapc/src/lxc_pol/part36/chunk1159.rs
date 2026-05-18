//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1159/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1159<F: Float>(t1036: F, t11311: F, t13738: F, t5856: F, t11496: F, t185: F, t9386: F, t11435: F, t129: F, t21778: F, t11434: F, t26331: F, t5544: F) -> (F, F, F, F) {
    let t34394 = t5856 * t11311 * t1036 * t13738;
    let t34397 = t185 * t9386 * t11496;
    let t34400 = t21778 * t129 * t11435;
    let t34403 = t11434 * t26331 * t5544;
    (t34394, t34397, t34400, t34403)
}
