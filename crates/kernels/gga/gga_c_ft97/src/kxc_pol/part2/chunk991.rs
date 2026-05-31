//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 991/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk991<F: Float>(t2766: F, t863: F, t4141: F, t2681: F, t309: F, t1212: F, t870: F, t2867: F, t4147: F, t8392: F, t2405: F, t4150: F) -> (F, F, F, F) {
    let t15365 = t2766 * t863;
    let t15366 = t15365 * t4141;
    let t15369 = t2681 * t309;
    let t15370 = t870 * t1212;
    let t15371 = t15370 * t2867;
    let t15372 = t15369 * t15371;
    let t15376 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t4147;
    let t15377 = t4150 * t2405;
    (t15366, t15372, t15376, t15377)
}
