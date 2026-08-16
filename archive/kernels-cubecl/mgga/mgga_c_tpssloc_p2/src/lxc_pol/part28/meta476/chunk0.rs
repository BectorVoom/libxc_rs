//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1689/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1689<F: Float>(t25373: F, t25374: F, t1530: F, t606: F, t25: F, t4303: F, t1408: F, t776: F, t868: F, t28: F, t870: F, t4255: F) -> (F, F, F, F, F, F, F) {
    let t25375 = t25373 * t25374;
    let t25377 = t606 * t1530;
    let t25381 = t25 * t4303;
    let t25385 = t1408 * t776;
    let t25392 = t1408 * t868;
    let t25891 = t870 * t28;
    let t25892 = t25891 * t4255;
    (t25375, t25377, t25381, t25385, t25392, t25891, t25892)
}
