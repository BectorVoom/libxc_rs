//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 630/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk630<F: Float>(t3450: F, t5855: F, t26999: F, t1882: F, t6692: F, t160: F, t6615: F, t379: F, t2221: F, t23571: F, t3478: F, t12968: F, t1359: F, t2178: F) -> (F, F, F, F, F, F, F, F) {
    let t27000 = t5855 * t3450;
    let t27001 = t26999 * t27000;
    let t27004 = t1882 * t6692;
    let t27006 = t160 * t6615;
    let t27007 = t27006 * t379;
    let t27008 = t2221 * t27007;
    let t27011 = t23571 * t3478;
    let t27012 = t12968 * t27011;
    let t27015 = t2178 * t1359;
    (t27000, t27001, t27004, t27007, t27008, t27011, t27012, t27015)
}
