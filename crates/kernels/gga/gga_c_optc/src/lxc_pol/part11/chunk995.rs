//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 995/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk995<F: Float>(t18023: F, t4290: F, t4289: F, t4282: F, t3245: F, t4300: F, t5101: F, t11900: F, t5249: F, t5256: F, t1495: F, t5239: F) -> (F, F, F, F, F, F, F, F) {
    let t18183 = t4290 * t18023;
    let t18184 = t4289 * t18183;
    let t18187 = t4282 * t18023;
    let t18188 = t3245 * t18187;
    let t18190 = t4300 * t5101;
    let t18191 = t11900 * t18190;
    let t18194 = t5249 * t5256;
    let t18197 = t5239 * t1495;
    (t18183, t18184, t18187, t18188, t18190, t18191, t18194, t18197)
}
