//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1483/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1483<F: Float>(t44878: F, t44943: F, t44999: F, t45066: F, t45133: F, t45186: F, t45246: F, t45311: F, t3609: F, t44927: F, t3623: F, t11880: F, t44690: F) -> (F, F, F, F) {
    let t45314 = t44878 + t44943 + t44999 + t45066 + t45133 + t45186 + t45246 + t45311;
    let t45320 = t44927 * t3609;
    let t45323 = t44927 * t3623;
    let t45326 = t44690 * t11880;
    (t45314, t45320, t45323, t45326)
}
