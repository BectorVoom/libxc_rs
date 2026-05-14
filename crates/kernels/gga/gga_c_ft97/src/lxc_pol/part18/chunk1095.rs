//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1095/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1095<F: Float>(t22759: F, t444: F, t3076: F, t22708: F, t408: F, t1609: F, t47: F, t9: F, t1624: F, t1593: F, t420: F, t422: F, t1710: F, t5532: F, t22675: F, t1737: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92872 = t22759 * t444;
    let t92873 = t3076 * t92872;
    let t92883 = t408 * t22708;
    let t92895 = t1609 * t47;
    let t92896 = t9 * t92895;
    let t92897 = t1624 * t92896;
    let t92899 = t420 * t422 * t1593;
    let t92920 = t1710 * t5532;
    let t92926 = t408 * t22675;
    let t92957 = t420 * t1737;
    (t92872, t92873, t92883, t92896, t92897, t92899, t92920, t92926, t92957)
}
