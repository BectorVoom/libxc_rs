//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 863/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk863<F: Float>(t1804: F, t1823: F, t5218: F, t7514: F, t1648: F, t5545: F, t1403: F, t1407: F, t1663: F, t1821: F, t587: F, t1769: F, t5548: F) -> (F, F, F, F, F) {
    let t16609 = F::new(64.0) / F::new(15.0) * t5218 * t7514 * t1804 * t1823;
    let t16611 = F::new(16.0) / F::new(9.0) * t1648 * t5545;
    let t16613 = t1663 * t1407 * t1403;
    let t16616 = F::new(16.0) / F::new(5.0) * t587 * t1821 * t16613;
    let t16620 = F::new(16.0) / F::new(15.0) * t587 * t5548 * t1769 * t1804;
    (t16609, t16611, t16613, t16616, t16620)
}
