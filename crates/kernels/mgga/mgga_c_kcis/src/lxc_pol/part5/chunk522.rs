//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 522/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk522<F: Float>(t228: F, t2532: F, t2535: F, t2541: F, t2627: F, t2764: F, t2766: F, t2771: F, t2772: F, t2789: F, t899: F, t906: F, t180: F, t296: F, t10: F, t251: F) -> (F, F, F, F, F) {
    let t2791 = t228 * t2764 - 2.0 * t2766 * t906 + 2.0 * t2771 * t2772 - t2789 * t899 - t2532 + t2535 - t2541 + t2627;
    let t2792 = t180 * t2791;
    let t2810 = t296 * t296;
    let t2811 = 1.0 / t2810;
    let t2820 = t10 * t251;
    (t2791, t2792, t2810, t2811, t2820)
}
