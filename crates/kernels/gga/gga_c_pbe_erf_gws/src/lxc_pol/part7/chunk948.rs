//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 948/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk948<F: Float>(t1416: F, t1620: F, t1809: F, t4901: F, t1413: F, t1642: F, t1733: F, t2677: F, t1697: F, t4367: F, t5002: F, t617: F) -> (F, F, F, F) {
    let t17591 = F::new(16.0) / F::new(15.0) * t1620 * t1809 * t4901 * t1416;
    let t17596 = F::new(16.0) / F::new(9.0) * t1620 * t2677 * t1733 * t1642 * t1413;
    let t17601 = F::new(32.0) / F::new(15.0) * t1620 * t1809 * t1733 * t1697 * t1413;
    let t17606 = F::new(64.0) / F::new(9.0) * t1620 * t2677 * t617 * t5002 * t4367;
    (t17591, t17596, t17601, t17606)
}
