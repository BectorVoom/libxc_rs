//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1158/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1158<F: Float>(t17060: F, t2367: F, t930: F, t2668: F, t42136: F, t4947: F, t17190: F, t2586: F, t940: F, t17034: F, t3917: F, t42111: F) -> (F, F, F, F) {
    let t51824 = t930 * t2367 * t17060;
    let t51827 = t2668 * t42136 * t4947;
    let t51903 = t940 * t2586 * t17190;
    let t51916 = t3917 * t42111 * t17034;
    (t51824, t51827, t51903, t51916)
}
