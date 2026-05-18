//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1048/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1048<F: Float>(t3099: F, t52: F, t7182: F, t7189: F, t938: F, t136866: F, t6427: F, t136968: F, t934: F, t25658: F, t32296: F, t115418: F, t136996: F) -> (F, F, F, F, F, F) {
    let t145353 = t52 * t7182 * t3099;
    let t145361 = t7189 * t938;
    let t145372 = t136866 * t6427;
    let t145376 = t136968 * t934;
    let t145379 = t32296 * t25658;
    let t145382 = t136996 * t115418;
    (t145353, t145361, t145372, t145376, t145379, t145382)
}
