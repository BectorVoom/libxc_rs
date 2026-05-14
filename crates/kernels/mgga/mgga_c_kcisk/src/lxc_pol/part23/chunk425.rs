//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 425/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk425<F: Float>(t2677: F, t2679: F, t140: F, t178: F, t190: F, t218: F, t167: F, t206: F, t116: F, t213: F) -> (F, F, F, F, F) {
    let t2680 = t2677 * t2679;
    let t2683 = t140 * t178 * t190;
    let t2685 = -0.10416666666666666667e-1 * t2680 + 0.99491666666666666664e-2 * t2683;
    let t2686 = t2685 * t218;
    let t2687 = t206 * t167;
    let t2689 = t116 * t213;
    (t2683, t2685, t2686, t2687, t2689)
}
