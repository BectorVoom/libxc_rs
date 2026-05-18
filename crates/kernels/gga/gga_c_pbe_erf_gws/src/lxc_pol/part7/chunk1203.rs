//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1203/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1203<F: Float>(t2120: F, t21385: F, t2105: F, t2112: F, t2124: F, t2387: F, t6701: F, t21011: F, t21350: F, t21355: F, t21359: F, t21361: F, t21366: F, t21378: F, t21382: F, t2277: F, t2343: F, t6195: F, t6282: F, t6366: F, t6524: F, t6609: F, t904: F, t929: F, t9482: F) -> (F, F, F) {
    let t21387 = t2120 * t21385 / F::new(96.0);
    let t21388 = t2105 * t2112;
    let t21395 = t2387 * t6701 * t2124 / F::new(16.0);
    let t21396 = -F::new(7.0) / F::new(96.0) * t21350 - t21355 + t21359 + F::new(35.0) / F::new(128.0) * t929 * t21361 * t904 * t21011 + t2277 * t9482 * t6609 * t21366 / F::new(64.0) - F::new(5.0) / F::new(64.0) * t2343 * t6366 * t6282 * t6524 - t21378 + t21382 - t21387 + t2277 * t9482 * t6195 * t21388 / F::new(64.0) - t21395;
    (t21387, t21395, t21396)
}
