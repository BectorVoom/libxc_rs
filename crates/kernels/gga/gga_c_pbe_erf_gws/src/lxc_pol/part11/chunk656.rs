//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 656/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk656<F: Float>(t362: F, t6552: F, t366: F, t899: F, t2209: F, t825: F, t346: F, t6158: F, t2251: F, t2299: F, t2276: F, t22: F, t4258: F) -> (F, F, F, F, F, F) {
    let t6553 = t6552 * t362;
    let t6555 = t899 * t6553 * t366;
    let t6560 = t825 * t2209;
    let t6566 = t6158 * t346;
    let t6578 = t2251 * t2299;
    let t6579 = t2276 * t6578;
    let t6587 = F::new(1.0) / t22 / t4258;
    (t6553, t6555, t6560, t6566, t6579, t6587)
}
