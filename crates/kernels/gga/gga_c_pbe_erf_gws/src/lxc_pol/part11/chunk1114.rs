//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1114/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1114<F: Float>(t41039: F, t41042: F, t3390: F, t16824: F, t186: F, t211: F, t41046: F, t41048: F, t10465: F, t12509: F, t5211: F, t41053: F) -> (F, F, F, F, F, F, F) {
    let t47794 = F::new(64.0) / F::new(45.0) * t41039;
    let t47795 = F::new(64.0) / F::new(45.0) * t41042;
    let t47796 = t3390 * t3390;
    let t47800 = F::new(16.0) / F::new(5.0) * t211 * t186 * t16824 * t47796;
    let t47801 = F::new(32.0) / F::new(15.0) * t41046;
    let t47802 = F::new(32.0) / F::new(15.0) * t41048;
    let t47805 = F::new(64.0) / F::new(15.0) * t5211 * t10465 * t12509;
    let t47806 = F::new(32.0) / F::new(45.0) * t41053;
    (t47794, t47795, t47800, t47801, t47802, t47805, t47806)
}
