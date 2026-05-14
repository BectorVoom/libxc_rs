//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 865/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk865<F: Float>(t1236: F, t14575: F, t3088: F, t1248: F, t980: F, t3930: F, t872: F, t3858: F, t880: F, t13326: F, t188: F, t3901: F, t14255: F, t317: F, t863: F, t3883: F, t852: F) -> (F, F, F, F, F, F, F, F) {
    let t14577 = t3088 * t1236 * t14575;
    let t14579 = t980 * t1248;
    let t14591 = t3930 * t872;
    let t14593 = t3858 * t880;
    let t14606 = 0.65854491829355115987e0 * t13326 * t188;
    let t14616 = t3901 * t880;
    let t14620 = 0.39512695097613069591e1 * t863 * t317 * t14255;
    let t14621 = t852 * t3883;
    (t14577, t14579, t14591, t14593, t14606, t14616, t14620, t14621)
}
