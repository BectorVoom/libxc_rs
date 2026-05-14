//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1065/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1065<F: Float>(t100: F, t369: F, t499: F, t8326: F, t110: F, t38482: F, t488: F, t8275: F, t1780: F, t1825: F, t3170: F, t463: F, t12553: F, t135: F, t131: F, t2030: F) -> (F, F, F, F, F, F, F, F) {
    let t47667 = t369 * t100;
    let t47759 = t8326 * t499;
    let t47768 = t38482 * t110;
    let t47799 = t8275 * t488;
    let t47809 = t1780 * t1825;
    let t47831 = t463 * t3170;
    let t48613 = t12553 * t135;
    let t48660 = t2030 * t131;
    (t47667, t47759, t47768, t47799, t47809, t47831, t48613, t48660)
}
