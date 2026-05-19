//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 457/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk457<F: Float>(t1281: F, t204: F, t334: F, t648: F, t824: F) -> (F, F, F) {
    let t2172 = t204 * t1281 * t334;
    let t2173 = F::cast_from(0.23744444444444444444e-1_f64) * t2172;
    let t2175 = t204 * t648 * t824;
    (t2172, t2173, t2175)
}
