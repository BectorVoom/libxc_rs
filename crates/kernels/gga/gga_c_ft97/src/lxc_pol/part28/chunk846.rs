//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 846/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk846<F: Float>(t1882: F, t32352: F, t32325: F, t358: F, t1570: F, t7165: F, t637: F, t7241: F, t1286: F, t1637: F, t7217: F, t497: F, t7211: F, t32053: F, t92: F, t32374: F, t376: F) -> (F, F, F, F, F, F, F, F, F) {
    let t137229 = t1882 * t32352;
    let t137231 = t32325 * t358;
    let t137236 = t7165 * t1570;
    let t137245 = t637 * t7241;
    let t137262 = 4.0 / 27.0 * t1286 * t1637 * t7217;
    let t137298 = t7165 * t497;
    let t137311 = t7211 * t497;
    let t137324 = t32053 * t92;
    let t137350 = t1286 * t376 * t32374;
    (t137229, t137231, t137236, t137245, t137262, t137298, t137311, t137324, t137350)
}
