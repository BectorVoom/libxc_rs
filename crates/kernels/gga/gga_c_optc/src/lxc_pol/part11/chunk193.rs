//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 193/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk193<F: Float>(t31: F, t4: F, t509: F, t27: F, t13: F, t1: F, t14: F, t3: F, t6: F) -> (F, F, F, F, F, F) {
    let t512 = F::cast_from(0.11073577833333333333e-2_f64) * t4 * t509 * t31;
    let t513 = t27 * t27;
    let t514 = F::new(1.0) / t513;
    let t515 = t13 * t514;
    let t517 = F::new(1.0) / t14 * t1;
    let t518 = t3 * t6;
    (t512, t513, t514, t515, t517, t518)
}
