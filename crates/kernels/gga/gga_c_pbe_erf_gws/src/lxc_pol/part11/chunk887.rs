//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 887/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk887<F: Float>(t16704: F, t1662: F, t1763: F, t5292: F, t56: F, t5175: F, t590: F, t1630: F, t1791: F, t5109: F, t642: F, t218: F, t5108: F) -> (F, F, F, F, F, F, F, F) {
    let t16705 = F::new(0.19591358024691358025e-1) * t16704;
    let t16712 = F::new(1.0) / t1662 / t1763;
    let t16738 = t56 * t5292;
    let t16739 = t1662 * t1662;
    let t16740 = F::new(1.0) / t16739;
    let t16782 = t590 * t5175;
    let t16797 = t1630 * t1791;
    let t16801 = t642 * t5109;
    let t16823 = F::new(1.0) / t5108 / t218;
    (t16705, t16712, t16738, t16740, t16782, t16797, t16801, t16823)
}
