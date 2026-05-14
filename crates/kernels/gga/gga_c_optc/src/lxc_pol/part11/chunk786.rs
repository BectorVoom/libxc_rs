//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 786/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk786<F: Float>(t43: F, t16225: F, t16231: F, t3365: F, t4565: F, t47: F, t6713: F, t1239: F, t4570: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t16235 = piecewise3(t44, 0.0, -8.0 / 27.0 * t6713 * t16225 + 4.0 / 3.0 * t3365 * t4565 + 4.0 / 3.0 * t47 * t16231);
    let t16236 = t4570 * t1239;
    (t16235, t16236)
}
