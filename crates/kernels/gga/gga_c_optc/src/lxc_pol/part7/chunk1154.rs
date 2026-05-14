//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1154/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1154<F: Float>(t26201: F, t2995: F, t1032: F, t8581: F, t8583: F, t26153: F, t3020: F, t8686: F, t1036: F, t8896: F, t1057: F, t3057: F, t3060: F, t1102: F, t26164: F, t3071: F, t8743: F) -> (F, F, F, F, F, F, F, F) {
    let t26203 = 12.0 * t26201 * t2995;
    let t26204 = t1032 * t8581;
    let t26206 = 0.38596378373162651572e3 * t26204 * t8583;
    let t26209 = 0.57894567559743977359e3 * t8686 * t26153 * t3020;
    let t26210 = t8896 * t1036;
    let t26212 = 4.0 * t26210 * t1057;
    let t26213 = t3057 * t3057;
    let t26214 = 1.0 / t26213;
    let t26216 = t3060 * t3060;
    let t26217 = 1.0 / t26216;
    let t26220 = 0.91080982599109921211e5 * t1102 * t26214 * t26164 * t26217;
    let t26222 = 0.35089340384731224426e1 * t8743 * t3071;
    (t26203, t26206, t26209, t26212, t26214, t26217, t26220, t26222)
}
