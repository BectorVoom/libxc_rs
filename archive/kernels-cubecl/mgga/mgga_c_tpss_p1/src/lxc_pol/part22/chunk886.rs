//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 886/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk886<F: Float>(t720: F, t7870: F, t2209: F, t712: F, t177: F, t185: F, t2213: F, t7813: F, t705: F, t7850: F, t169: F, t2271: F) -> (F, F, F, F, F, F, F) {
    let t7871 = t7870 * t720;
    let t7875 = F::cast_from(1.0_f64) / t2209 / t712;
    let t7876 = t177 * t7875;
    let t7878 = F::cast_from(1.0_f64) / t2213 / t185;
    let t7879 = t7813 * t7878;
    let t7882 = t7850 * t705;
    let t7886 = F::cast_from(1.0_f64) / t2271 / t169;
    (t7871, t7875, t7876, t7878, t7879, t7882, t7886)
}
