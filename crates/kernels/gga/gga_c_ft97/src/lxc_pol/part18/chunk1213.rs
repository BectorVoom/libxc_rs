//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1213/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1213<F: Float>(t108: F, t25846: F, t22914: F, t25574: F, t11437: F, t11449: F, t1564: F, t1643: F, t1651: F, t22896: F, t22907: F, t22908: F, t22922: F, t25577: F, t25609: F, t25611: F, t25616: F, t25622: F, t25861: F, t3052: F, t378: F, t379: F, t5495: F, t5501: F, t5618: F, t6414: F, t7793: F, t93915: F, t93923: F, t93925: F) -> (F,) {
    let t101983 = t25846 * t108;
    let t102018 = t22914 * t25574 / 27.0;
    let t102021 = -t5501 * t1564 * t101983 * t379 / 9.0 - 2.0 / 9.0 * t25577 * t1564 * t22922 * t3052 + t6414 * t22896 / 3.0 - t5501 * t25609 * t25616 * t11437 / 3.0 + 2.0 / 9.0 * t5501 * t378 * t5618 * t25611 + t93915 / 27.0 - t5501 * t1564 * t25861 * t1651 / 18.0 - t5501 * t7793 * t25861 * t1643 / 27.0 + t5495 * t25622 / 3.0 + t5501 * t22907 * t22908 * t11449 / 9.0 + t102018 + t93923 / 27.0 + t93925 / 54.0;
    (t102021,)
}
