//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 628/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk628<F: Float>(t1221: F, t8232: F, t1242: F, t2399: F, t89: F, t2681: F, t309: F, t1212: F, t870: F, t10580: F, t312: F, t9570: F) -> (F, F, F, F, F, F) {
    let t15318 = t8232 * t1221;
    let t15329 = t89 * t2399 * t1242;
    let t15369 = t2681 * t309;
    let t15370 = t870 * t1212;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    (t15318, t15329, t15369, t15370, t15385, t15386)
}
