//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 798/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk798<F: Float>(t2535: F, t919: F, t9497: F, t1084: F, t3717: F, t2657: F, t2660: F, t9019: F, t2721: F, t3103: F, t2255: F, t2636: F) -> (F, F, F, F, F, F, F) {
    let t9499 = t2535 * t919 * t9497;
    let t9501 = t1084 * t3717;
    let t9502 = t9501 * t2657;
    let t9504 = t2660 * t9019;
    let t9505 = t9504 * t2657;
    let t9507 = t2721 * t3103;
    let t9508 = t2636 * t2255;
    (t9499, t9501, t9502, t9504, t9505, t9507, t9508)
}
