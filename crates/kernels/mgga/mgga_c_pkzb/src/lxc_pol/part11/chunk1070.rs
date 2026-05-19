//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1070/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1070<F: Float>(t127: F, t16942: F, t79: F, t1613: F, t16588: F, t541: F, t555: F, t146: F, t1540: F, t155: F, t52: F, t95: F) -> (F, F, F) {
    let t16946 = F::new(840.0) * t79 / t16942 * t127;
    let t16950 = F::cast_from(0.35089341735807877242e1_f64) * t555 * t1613 * t16588 * t541;
    let t17026 = F::new(455.0) / F::new(243.0) * t146 / t52 / t1540 * t95 * t155;
    (t16946, t16950, t17026)
}
