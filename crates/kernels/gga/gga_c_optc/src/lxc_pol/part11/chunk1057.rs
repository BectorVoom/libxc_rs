//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1057/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1057<F: Float>(t2910: F, t27071: F, t490: F, t492: F, t496: F, t1244: F, t40: F, t6524: F, t108: F, t1256: F, t176: F, t203: F, t6599: F) -> (F, F, F, F) {
    let t28030 = t2910 * t2910;
    let t28031 = F::new(1.0) / t28030;
    let t28109 = F::new(40.0) / F::new(81.0) * t490 * t492 * t27071 * t496;
    let t28141 = t40 * t1244 * t6524;
    let t28145 = t176 * t6599 * t1256 * t108 * t203;
    (t28031, t28109, t28141, t28145)
}
