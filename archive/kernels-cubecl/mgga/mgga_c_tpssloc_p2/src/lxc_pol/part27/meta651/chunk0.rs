//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2264/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2264<F: Float>(t2018: F, t3734: F, t1983: F, t7687: F, t26062: F, t645: F, t72: F, t26066: F, t2307: F, t7431: F, t26012: F, t6505: F) -> (F, F, F, F, F) {
    let t90065 = t3734 * t2018;
    let t90068 = F::cast_from(6.0_f64) * t1983 * t90065 * t7687;
    let t90072 = t72 * t26062 * t645;
    let t90076 = t72 * t26066 * t645;
    let t90080 = t72 * t7431 * t2307;
    let t90087 = t6505 * t26012;
    (t90068, t90072, t90076, t90080, t90087)
}
