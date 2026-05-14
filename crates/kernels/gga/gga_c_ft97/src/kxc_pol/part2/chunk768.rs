//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 768/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk768<F: Float>(t1123: F, t626: F, t701: F, t13625: F, t13629: F, t13633: F, t13636: F, t13637: F, t13639: F, t13643: F, t13645: F, t9637: F, t9645: F, t13623: F, t1095: F, t694: F) -> (F, F, F) {
    let t13647 = t626 * t1123;
    let t13648 = t701 * t13647;
    let t13650 = -0.51074886703703703704e-1 * t13625 - 0.42562405586419753086e-2 * t9645 + 0.2979368391049382716e-1 * t13629 - 0.12768721675925925926e-1 * t13633 - t13636 - 0.17024962234567901235e-1 * t13637 - 0.2269994964609053498e-1 * t13639 + 0.1134997482304526749e-1 * t13643 + t9637 + 0.3404992446913580247e-1 * t13645 - 0.14187468528806584362e-2 * t13648;
    let t13651 = t13623 + t13650;
    let t13654 = t694 * t1095;
    (t13648, t13651, t13654)
}
