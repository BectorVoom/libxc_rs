//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 885/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk885<F: Float>(t16853: F, t16684: F, t2559: F, t587: F, t1820: F, t4952: F, t562: F, t7435: F, t1416: F, t1815: F, t4896: F, t639: F) -> (F, F, F, F) {
    let t16854 = F::new(32.0) / F::new(15.0) * t16853;
    let t16857 = F::new(16.0) / F::new(27.0) * t587 * t2559 * t16684;
    let t16861 = F::new(256.0) / F::new(81.0) * t1820 * t7435 * t4952 * t562;
    let t16865 = F::new(8.0) / F::new(15.0) * t639 * t1815 * t4896 * t1416;
    (t16854, t16857, t16861, t16865)
}
