//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 453/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk453<F: Float>(t1573: F, t324: F, t1541: F, t1548: F, t1551: F, t1554: F, t945: F, t948: F) -> (F, F) {
    let t1574 = t1573 * t324;
    let t1580 = F::cast_from(0.258925e1_f64) * t1548 - t945 - F::cast_from(0.301925e0_f64) * t1541 + F::cast_from(0.16504875e0_f64) * t1551 - t948 - F::cast_from(0.82785e-1_f64) * t1554;
    (t1574, t1580)
}
