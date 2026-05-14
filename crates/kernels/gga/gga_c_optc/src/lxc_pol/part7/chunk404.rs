//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 404/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk404<F: Float>(t111: F, t2010: F, t1928: F, t5: F, t629: F, t1948: F, t105: F, t692: F, t635: F) -> (F, F, F, F, F, F) {
    let t2011 = t111 * t2010;
    let t2012 = t5 * t1928;
    let t2013 = t629 * t2012;
    let t2017 = t629 * t5 * t1948;
    let t2020 = t105 * t692;
    let t2021 = t2020 * t635;
    (t2011, t2012, t2013, t2017, t2020, t2021)
}
