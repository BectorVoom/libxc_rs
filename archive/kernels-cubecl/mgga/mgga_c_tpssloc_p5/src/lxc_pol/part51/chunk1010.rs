//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1010/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1010<F: Float>(t25373: F, t25374: F, t1530: F, t606: F, t25: F, t4303: F, t1408: F, t776: F, t868: F, t1877: F, t1915: F, t2219: F) -> (F, F, F, F, F, F) {
    let t25375 = t25373 * t25374;
    let t25377 = t606 * t1530;
    let t25381 = t25 * t4303;
    let t25385 = t1408 * t776;
    let t25392 = t1408 * t868;
    let t25397 = t1877 * t1915 * t2219;
    (t25375, t25377, t25381, t25385, t25392, t25397)
}
