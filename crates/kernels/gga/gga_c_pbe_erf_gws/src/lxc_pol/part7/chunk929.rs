//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 929/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk929<F: Float>(t17363: F, t1620: F, t1621: F, t1724: F, t5500: F, t1640: F, t1643: F, t16986: F, t639: F, t1891: F, t642: F, t1648: F, t5510: F) -> (F, F, F, F, F) {
    let t17364 = F::new(32.0) / F::new(15.0) * t17363;
    let t17368 = F::new(8.0) / F::new(5.0) * t1620 * t1621 * t5500 * t1724;
    let t17372 = F::new(4.0) / F::new(9.0) * t639 * t1640 * t1643 * t16986;
    let t17376 = F::new(8.0) / F::new(15.0) * t639 * t642 * t1891 * t16986;
    let t17378 = F::new(32.0) / F::new(15.0) * t1648 * t5510;
    (t17364, t17368, t17372, t17376, t17378)
}
