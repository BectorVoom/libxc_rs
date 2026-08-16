//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1061/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1061<F: Float>(t41305: F, t41307: F, t43841: F, t43849: F, t43858: F, t43861: F, t43864: F, t43875: F, t43879: F, t43881: F, t43883: F, t43884: F, t43885: F, t43886: F, t43887: F, t43888: F, t43889: F, t43892: F, t43893: F, t47408: F) -> F {
    let t51146 = t43841 - t43849 - t43858 + t43861 + t43864 + t43875 - t43879 + F::cast_from(0.76685851907841499352e0_f64) * t43881 + t43883 + t43884 - t43885 - t43886 + t43887 - t43888 - t43889 + F::cast_from(0.59584149919750711115e-1_f64) * t41305 - F::cast_from(0.89376224879626066675e-1_f64) * t41307 + t43892 + t43893 - t47408;
    t51146
}
