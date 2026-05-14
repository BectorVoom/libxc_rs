//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 719/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk719<F: Float>(t1212: F, t870: F, t2867: F, t15369: F, t4147: F, t8392: F, t2405: F, t4150: F, t4139: F, t4257: F, t4262: F, t10580: F, t309: F, t312: F, t9570: F, t13863: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15370 = t870 * t1212;
    let t15371 = t15370 * t2867;
    let t15372 = t15369 * t15371;
    let t15376 = 2.0 / 27.0 * t8392 * t4147;
    let t15377 = t4150 * t2405;
    let t15378 = t4139 * t15377;
    let t15382 = 2.0 / 27.0 * t8392 * t4257;
    let t15384 = 2.0 / 27.0 * t8392 * t4262;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    let t15387 = t15386 * t13863;
    (t15371, t15372, t15376, t15377, t15378, t15382, t15384, t15385, t15387)
}
