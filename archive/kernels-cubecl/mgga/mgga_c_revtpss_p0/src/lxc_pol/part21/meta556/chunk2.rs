//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2246/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2246<F: Float>(t127: F, t371: F, t5318: F, t1235: F, t1803: F, t3670: F, t3685: F, t5373: F, t140: F, t5368: F, t1222: F, t3624: F, t5436: F) -> (F, F, F, F, F, F) {
    let t17435 = t371 * t127 * t5318;
    let t17437 = F::cast_from(0.28582678745379824648e-3_f64) * t1235 * t17435;
    let t17438 = t3670 * t1803;
    let t17444 = t5373 * t3685 / F::cast_from(162.0_f64);
    let t17445 = t140 * t5368;
    let t17447 = t1222 * t17445 / F::cast_from(432.0_f64);
    let t17448 = t5436 * t3624;
    (t17435, t17437, t17438, t17444, t17447, t17448)
}
