//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 635/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk635<F: Float>(t39: F, t5772: F, t5773: F, t505: F, t96: F, t1235: F, t125: F, t128: F, t2: F, t512: F, t131: F, t120: F, t133: F, t1365: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5776 = F::new(0.19486833333333333333e1) * t5772 * t5773 * t39;
    let t5825 = F::new(1.0) / t505 / t96;
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    let t5836 = F::new(0.32645333333333333334e0) * t5832 * t5833 * t39;
    let t5852 = t512 * t512;
    let t5853 = F::new(1.0) / t5852;
    let t5854 = t131 * t5853;
    let t5863 = F::new(0.89405814814814814813e0) * t133 * t1365 * t120;
    (t5776, t5825, t5832, t5833, t5836, t5852, t5853, t5854, t5863)
}
