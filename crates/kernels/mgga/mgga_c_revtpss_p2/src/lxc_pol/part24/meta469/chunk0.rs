//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1446/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1446<F: Float>(t18349: F, t2689: F, t124: F, t5977: F, t10760: F, t18409: F, t9794: F, t18414: F, t40799: F, t18418: F, t18643: F, t40731: F) -> (F, F, F, F, F, F) {
    let t61924 = t2689 * t18349;
    let t61956 = t124 * t5977;
    let t61981 = t10760 * t9794 * t18409;
    let t62012 = t40799 * t9794 * t18414;
    let t62015 = t10760 * t9794 * t18418;
    let t62029 = t40731 * t18643;
    (t61924, t61956, t61981, t62012, t62015, t62029)
}
