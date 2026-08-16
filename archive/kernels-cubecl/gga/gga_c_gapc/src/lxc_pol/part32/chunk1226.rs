//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1226/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1226<F: Float>(t128: F, t3141: F, t33655: F, t5541: F, t583: F, t11492: F, t34468: F, t11317: F, t2973: F, t3140: F, t34040: F, t27935: F, t27940: F) -> (F, F, F, F, F) {
    let t35132 = t5541 * t33655 * t3141 * t583 * t128;
    let t35135 = t34468 * t11492;
    let t35137 = t2973 * t11317;
    let t35139 = t34040 * t3140;
    let t35141 = t27935 * t35139 * t27940;
    (t35132, t35135, t35137, t35139, t35141)
}
