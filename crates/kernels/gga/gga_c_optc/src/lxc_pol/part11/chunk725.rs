//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 725/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk725<F: Float>(t1135: F, t9: F, t22: F, t3145: F, t8425: F, t8428: F, t449: F, t2849: F, t8414: F, t1772: F, t310: F, t448: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8446 = t9 * t1135;
    let t8459 = t22 * t3145;
    let t8482 = t8425 * t8428;
    let t8487 = t9 * t449;
    let t8511 = t1135 * t2849;
    let t8516 = t3145 * t8414;
    let t8528 = t1772 * t449;
    let t8529 = t310 * t8528;
    let t8531 = F::new(0.80492236016562572729e-3) * t448 * t8529;
    (t8446, t8459, t8482, t8487, t8511, t8516, t8528, t8529, t8531)
}
