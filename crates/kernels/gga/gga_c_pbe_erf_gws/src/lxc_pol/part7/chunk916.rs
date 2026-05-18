//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 916/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk916<F: Float>(t1651: F, t5287: F, t587: F, t5018: F, t5394: F, t1620: F, t1809: F, t5033: F, t617: F, t1815: F, t639: F, t661: F) -> (F, F, F, F) {
    let t17207 = t587 * t1651 * t5287;
    let t17208 = F::new(32.0) / F::new(135.0) * t17207;
    let t17210 = t587 * t5018 * t5394;
    let t17211 = F::new(32.0) / F::new(15.0) * t17210;
    let t17215 = F::new(64.0) / F::new(15.0) * t1620 * t1809 * t5033 * t617;
    let t17219 = F::new(32.0) / F::new(15.0) * t639 * t1815 * t5033 * t661;
    (t17208, t17211, t17215, t17219)
}
