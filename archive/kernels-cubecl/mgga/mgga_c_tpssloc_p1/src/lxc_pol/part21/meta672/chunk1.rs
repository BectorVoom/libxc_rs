//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2477/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2477<F: Float>(t3609: F, t44927: F, t3623: F, t11880: F, t44690: F, t11913: F, t11604: F, t496: F, t68: F, t107: F, t9576: F, t2585: F, t667: F) -> (F, F, F, F, F, F, F) {
    let t45320 = t44927 * t3609;
    let t45323 = t44927 * t3623;
    let t45326 = t44690 * t11880;
    let t45329 = t44690 * t11913;
    let t45349 = F::cast_from(1.0_f64) / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45421 = F::cast_from(2618.0_f64) / F::cast_from(81.0_f64) * t9576 * t107;
    let t45422 = t2585 * t667;
    (t45320, t45323, t45326, t45329, t45350, t45421, t45422)
}
