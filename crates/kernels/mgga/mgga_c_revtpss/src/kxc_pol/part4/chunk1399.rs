//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1399/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1399<F: Float>(t17730: F, t5051: F, t3626: F, t3566: F, t489: F, t17728: F, t1121: F, t1774: F, t3584: F, t471: F, t5351: F, t3720: F) -> (F, F, F, F) {
    let t17731 = t5051 * t17730;
    let t17732 = t3626 * t17731;
    let t17735 = t3566 * t489;
    let t17736 = t17735 * t17728;
    let t17737 = t1774 * t1121;
    let t17738 = t17737 * t17730;
    let t17739 = t3626 * t17738;
    let t17742 = t471 * t3584;
    let t17743 = t5351 * t17742;
    let t17744 = t3720 * t17743;
    (t17732, t17736, t17739, t17744)
}
