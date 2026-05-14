//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 815/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk815<F: Float>(t1627: F, t5005: F, t1624: F, t16649: F, t1820: F, t5018: F, t5308: F, t16684: F, t2559: F, t587: F, t4952: F, t562: F, t7435: F, t1416: F, t1815: F, t4896: F, t639: F) -> (F, F, F, F, F, F) {
    let t16849 = 32.0 / 9.0 * t1627 * t5005;
    let t16851 = 16.0 / 5.0 * t16649 * t1624;
    let t16853 = t1820 * t5018 * t5308;
    let t16854 = 32.0 / 15.0 * t16853;
    let t16857 = 16.0 / 27.0 * t587 * t2559 * t16684;
    let t16861 = 256.0 / 81.0 * t1820 * t7435 * t4952 * t562;
    let t16865 = 8.0 / 15.0 * t639 * t1815 * t4896 * t1416;
    (t16849, t16851, t16854, t16857, t16861, t16865)
}
