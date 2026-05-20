//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1400/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1400<F: Float>(t159: F, t793: F, t1493: F, t76: F, t1518: F, t94: F, t93: F, t587: F, t65: F) -> (F, F, F, F, F) {
    let t7021 = t793 * t159;
    let t7719 = t76 * t1493;
    let t7732 = t94 * t1518;
    let t7889 = t93 * t1518;
    let t8779 = F::new(1.0) / t65 / t587;
    (t7021, t7719, t7732, t7889, t8779)
}
