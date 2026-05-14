//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1040/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1040<F: Float>(t34503: F, t9256: F, t26007: F, t3708: F, t9304: F, t11455: F, t9325: F, t11312: F, t4940: F, t11320: F, t1875: F, t5190: F, t1765: F, t3670: F, t11391: F, t3163: F) -> (F, F, F, F, F, F, F) {
    let t34846 = t34503 * t9256;
    let t34849 = t9304 * t3708 * t26007;
    let t34851 = t11455 * t9325;
    let t34853 = t11312 * t4940;
    let t34856 = t1875 * t11320 * t5190;
    let t34858 = t3670 * t1765;
    let t34860 = t11391 * t3163;
    (t34846, t34849, t34851, t34853, t34856, t34858, t34860)
}
