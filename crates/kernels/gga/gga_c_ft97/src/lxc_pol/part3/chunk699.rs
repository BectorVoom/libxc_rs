//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 699/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk699<F: Float>(t2253: F, t3655: F, t3539: F, t604: F, t1882: F, t3324: F, t3327: F, t3320: F, t3339: F, t9065: F, t8796: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12240 = F::new(2.0) / F::new(3.0) * t2253 * t3655;
    let t12277 = t3539 * t604;
    let t12306 = t1882 * t3324;
    let t12307 = t12306 / F::new(27.0);
    let t12308 = t1882 * t3327;
    let t12309 = F::new(2.0) / F::new(27.0) * t12308;
    let t12310 = t1882 * t3320;
    let t12311 = F::new(2.0) / F::new(81.0) * t12310;
    let t12327 = t1882 * t3339;
    let t12328 = t12327 / F::new(27.0);
    let t12343 = F::new(4.0) / F::new(27.0) * t9065;
    let t12346 = F::new(4.0) / F::new(81.0) * t8796;
    (t12240, t12277, t12306, t12307, t12308, t12309, t12310, t12311, t12327, t12328, t12343, t12346)
}
