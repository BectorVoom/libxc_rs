//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 840/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk840<F: Float>(t2367: F, t2634: F, t930: F, t7406: F, t914: F, t2601: F, t7178: F, t2270: F, t2723: F, t2722: F, t2274: F, t2813: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8024 = t2367 * t2634;
    let t8025 = t930 * t8024;
    let t8027 = t914 * t7406;
    let t8036 = t2601 * t7178;
    let t8037 = t914 * t8036;
    let t8040 = t2270 * t2723;
    let t8041 = t2722 * t8040;
    let t8044 = t2723 * t2274;
    let t8045 = t2813 * t8044;
    (t8024, t8025, t8027, t8036, t8037, t8040, t8041, t8044, t8045)
}
