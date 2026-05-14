//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 742/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk742<F: Float>(t13545: F, t7129: F, t2508: F, t2586: F, t3650: F, t13492: F, t7137: F, t11613: F, t7696: F, t11608: F, t2530: F, t2580: F, t11603: F, t1897: F, t7068: F, t7226: F) -> (F, F, F, F, F, F, F, F) {
    let t45072 = 0.53833683610995569986e-1 * t7129 * t13545;
    let t45077 = 0.53833683610995569986e-1 * t2508 * t3650 * t2586;
    let t45079 = 0.12304841968227558854e0 * t7137 * t13492;
    let t45083 = 0.92286314761706691403e-1 * t7129 * t13492;
    let t45086 = 0.92286314761706691403e-1 * t2508 * t11613 * t7696;
    let t45087 = t11608 * t2530;
    let t45090 = 0.15381052460284448567e-1 * t2508 * t2580 * t45087;
    let t45101 = 0.46143157380853345701e-1 * t1897 * t7226 * t11603 * t7068;
    (t45072, t45077, t45079, t45083, t45086, t45087, t45090, t45101)
}
