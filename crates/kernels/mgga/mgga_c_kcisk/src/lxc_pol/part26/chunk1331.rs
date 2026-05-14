//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1331/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1331<F: Float>(t110593: F, t110595: F, t114199: F, t114351: F, t119290: F, t119293: F, t119298: F, t119302: F, t119305: F, t119308: F, t119311: F, t119313: F, t32008: F, t33384: F, t33588: F, t9809: F) -> (F,) {
    let t119315 = 0.20833333333333333334e-1 * t114199 * t9809 - 0.46296296296296296297e-2 * t114351 + 0.20833333333333333334e-1 * t33384 * t33588 + 0.22109259259259259259e-2 * t119290 - 0.88437037037037037035e-2 * t119293 + 0.23148148148148148149e-2 * t110593 + 0.23148148148148148149e-2 * t110595 - 0.40208333333333333333e-2 * t32008 * t119298 - 0.49745833333333333332e-2 * t119302 + 0.16581944444444444444e-2 * t119305 + 0.16581944444444444444e-2 * t119308 - 0.55273148148148148147e-3 * t119311 - 0.36848765432098765431e-3 * t119313;
    (t119315,)
}
