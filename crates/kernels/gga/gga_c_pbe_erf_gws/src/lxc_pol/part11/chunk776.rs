//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 776/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk776<F: Float>(t12623: F, t639: F, t1022: F, t3465: F, t2677: F, t1620: F, t3429: F, t995: F, t1821: F, t1820: F, t1017: F, t1827: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12625 = F::new(4.0) / F::new(9.0) * t639 * t12623;
    let t12626 = t3465 * t1022;
    let t12627 = t2677 * t12626;
    let t12629 = F::new(8.0) / F::new(9.0) * t1620 * t12627;
    let t12630 = t3429 * t995;
    let t12631 = t1821 * t12630;
    let t12633 = F::new(8.0) / F::new(15.0) * t1820 * t12631;
    let t12634 = t3429 * t1017;
    let t12635 = t1827 * t12634;
    (t12625, t12626, t12627, t12629, t12630, t12631, t12633, t12634, t12635)
}
