//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1181/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1181<F: Float>(t197: F, t3338: F, t161: F, t25893: F, t6520: F, t23763: F, t10215: F, t158: F, t475: F, t6508: F, t25722: F, t4261: F, t9074: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31730 = t197 * t3338;
    let t31731 = t31730 * t161;
    let t31735 = t25893 * t6520;
    let t31737 = F::new(0.18970004423784099733e-1) * t23763 * t31735;
    let t31740 = t158 * t10215;
    let t31747 = t3338 * t475;
    let t31748 = t6508 * t31747;
    let t31752 = t6508 * t25722;
    let t31754 = t9074 * t4261 * t31752;
    (t31730, t31731, t31735, t31737, t31740, t31747, t31748, t31752, t31754)
}
