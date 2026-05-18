//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 887/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk887<F: Float>(t1413: F, t1642: F, t1724: F, t5522: F, t639: F, t1464: F, t671: F, t1457: F, t4892: F, t5129: F, t587: F, t1820: F, t4919: F, t5125: F) -> (F, F, F, F, F) {
    let t16874 = F::new(8.0) / F::new(9.0) * t639 * t5522 * t1724 * t1642 * t1413;
    let t16876 = F::new(0.44134814814814814812e-2) * t1464 * t671;
    let t16877 = t1457 * t671;
    let t16880 = t587 * t5129 * t4892;
    let t16881 = F::new(32.0) / F::new(45.0) * t16880;
    let t16883 = t1820 * t5125 * t4919;
    (t16874, t16876, t16877, t16881, t16883)
}
