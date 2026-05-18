//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1158/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1158<F: Float>(t1017: F, t10843: F, t12480: F, t12635: F, t12794: F, t12810: F, t1809: F, t1820: F, t1821: F, t1827: F, t2615: F, t30740: F, t32629: F, t3342: F, t3415: F, t42187: F, t42189: F, t42191: F, t42204: F, t47983: F, t587: F, t639: F, t7130: F) -> F {
    let t48423 = F::new(32.0) / F::new(15.0) * t42187 + F::new(32.0) / F::new(27.0) * t42189 + F::new(64.0) / F::new(45.0) * t42191 - F::new(32.0) / F::new(15.0) * t1820 * t1821 * t30740 * t3342 - F::new(64.0) / F::new(15.0) * t7130 * t12810 + F::new(16.0) / F::new(15.0) * t587 * t1827 * t32629 * t3342 - F::new(32.0) / F::new(15.0) * t10843 * t3415 + F::new(16.0) / F::new(5.0) * t639 * t1809 * t47983 - F::new(16.0) / F::new(15.0) * t2615 * t12635 - F::new(16.0) / F::new(45.0) * t587 * t1827 * t12480 * t1017 - F::new(32.0) / F::new(15.0) * t2615 * t12794 + F::new(64.0) / F::new(45.0) * t42204;
    t48423
}
