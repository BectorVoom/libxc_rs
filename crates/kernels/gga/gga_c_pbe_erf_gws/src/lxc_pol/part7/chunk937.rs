//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 937/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk937<F: Float>(t1648: F, t5010: F, t155: F, t188: F, t1820: F, t1887: F, t1620: F, t5493: F, t5501: F, t1885: F, t5175: F, t5177: F, t562: F) -> (F, F, F, F) {
    let t17469 = F::new(32.0) / F::new(9.0) * t1648 * t5010;
    let t17470 = t155 * t188;
    let t17472 = t1820 * t17470 * t1887;
    let t17473 = F::new(32.0) / F::new(45.0) * t17472;
    let t17475 = t1620 * t5493 * t5501;
    let t17476 = F::new(32.0) / F::new(15.0) * t17475;
    let t17481 = F::new(32.0) / F::new(5.0) * t1820 * t1885 * t5175 * t5177 * t562;
    (t17469, t17473, t17476, t17481)
}
