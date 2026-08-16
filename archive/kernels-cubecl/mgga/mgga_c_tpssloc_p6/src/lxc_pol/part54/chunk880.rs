//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 880/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk880<F: Float>(t2095: F, t8643: F, t1983: F, t1873: F, t7230: F, t2039: F) -> (F, F, F, F) {
    let t8644 = t2095 * t8643;
    let t8645 = t1983 * t8644;
    let t8654 = F::cast_from(0.135e2_f64) * t7230 * t1873;
    let t8657 = t2039 * t1873;
    (t8644, t8645, t8654, t8657)
}
