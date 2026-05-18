//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1009/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1009<F: Float>(t1849: F, t22100: F, t601: F, t6424: F, t1847: F, t588: F, t6419: F, t6347: F, t6405: F, t2002: F, t518: F, t596: F, t84: F) -> (F, F, F, F) {
    let t22103 = F::new(0.61523382126046769581e4) * t601 * t6424 * t1849 * t22100;
    let t22107 = F::new(0.46785787179641632568e1) * t601 * t1847 * t6419 * t588;
    let t22111 = F::new(0.62336721237753107879e3) * t601 * t6405 * t1849 * t6347;
    let t22115 = F::new(0.18989760778855128827e-2) * t596 * t518 * t2002 * t84;
    (t22103, t22107, t22111, t22115)
}
