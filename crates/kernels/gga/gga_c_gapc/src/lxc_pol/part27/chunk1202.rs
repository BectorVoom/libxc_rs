//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1202/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1202<F: Float>(t34503: F, t9256: F, t26007: F, t3708: F, t9304: F, t11455: F, t9325: F, t11312: F, t4940: F, t11320: F, t1875: F, t5190: F) -> (F, F, F, F, F) {
    let t34846 = t34503 * t9256;
    let t34849 = t9304 * t3708 * t26007;
    let t34851 = t11455 * t9325;
    let t34853 = t11312 * t4940;
    let t34856 = t1875 * t11320 * t5190;
    (t34846, t34849, t34851, t34853, t34856)
}
