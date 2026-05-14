//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 963/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk963<F: Float>(t1242: F, t26170: F, t1235: F, t1248: F, t13603: F, t7736: F, t3979: F, t7740: F, t20302: F, t20406: F, t26130: F, t26141: F, t26144: F, t26147: F, t26153: F, t1249: F, t25313: F) -> (F, F, F, F, F, F) {
    let t26171 = t1242 * t26170;
    let t26173 = t1235 * t26170;
    let t26176 = t1248 * t13603 * t7736;
    let t26179 = t1248 * t3979 * t7740;
    let t26185 = -0.5477111111111111111e-1 * t26130 + 0.43816888888888888888e0 * t20406 + 0.39862222222222222222e0 * t20302 + 0.3071625e0 * t26171 + 0.1898925e1 * t26173 + 0.36514074074074074073e-1 * t26176 - 0.21908444444444444444e0 * t26179 - 0.33218518518518518518e0 * t26141 + 0.11958666666666666667e1 * t26144 - 0.79724444444444444444e0 * t26147 - 0.17938e1 * t26153;
    let t26195 = t1248 * t1249 * t25313;
    (t26171, t26173, t26176, t26179, t26185, t26195)
}
