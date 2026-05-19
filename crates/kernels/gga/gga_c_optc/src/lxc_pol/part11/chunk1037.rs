//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1037/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1037<F: Float>(t24565: F, t8125: F, t2672: F, t2748: F, t24502: F, t330: F, t310: F, t312: F, t3648: F, t307: F, t23573: F, t24391: F) -> (F, F, F, F, F, F, F) {
    let t24566 = t8125 * t24565;
    let t24568 = t2672 * t2672;
    let t24574 = t2748 * t24565;
    let t24583 = t330 * t24502;
    let t24599 = t310 * t3648 * t312;
    let t24601 = F::cast_from(0.18781521737197933637e-2_f64) * t307 * t24599;
    let t24619 = t24391 * t23573;
    (t24566, t24568, t24574, t24583, t24599, t24601, t24619)
}
