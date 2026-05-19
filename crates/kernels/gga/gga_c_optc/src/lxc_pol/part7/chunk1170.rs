//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1170/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1170<F: Float>(t23682: F, t23685: F, t23660: F, t23664: F, t23667: F, t23670: F, t23673: F, t23676: F, t23679: F, t23928: F, t23931: F, t23933: F, t23936: F, t23938: F) -> F {
    let t24287 = F::cast_from(0.31003950617283950618e1_f64) * t23682;
    let t24288 = F::cast_from(0.13388493827160493828e1_f64) * t23685;
    let t24294 = F::cast_from(0.23917333333333333333e1_f64) * t23660 - F::new(0.295764e1) * t23664 + F::cast_from(0.65725333333333333332e0_f64) * t23667 + F::cast_from(0.71752000000000000001e1_f64) * t23670 - F::cast_from(0.79724444444444444444e0_f64) * t23673 - F::cast_from(0.19931111111111111111e1_f64) * t23676 - F::new(0.107628e2) * t23679 + t24287 + t24288 + F::new(0.1898925e1) * t23928 + F::new(0.85451625e1) * t23931 - F::new(0.379785e1) * t23933 - F::new(0.46074375e0) * t23936 + F::new(0.614325e0) * t23938;
    t24294
}
