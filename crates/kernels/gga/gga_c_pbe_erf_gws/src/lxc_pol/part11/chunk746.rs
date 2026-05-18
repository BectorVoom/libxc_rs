//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 746/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk746<F: Float>(t12323: F, t171: F, t9763: F, t6968: F, t7986: F, t10017: F, t7988: F, t7990: F, t4499: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F) -> (F, F, F, F, F, F, F, F) {
    let t12324 = t171 * t12323;
    let t12332 = F::new(0.54934665110259479823e-3) * t9763;
    let t12333 = F::new(0.32530742648344572643e-1) * t6968;
    let t12334 = F::new(60.0) * t7986;
    let t12335 = F::new(3.0) * t10017;
    let t12336 = F::new(96.0) * t7988;
    let t12337 = F::new(24.0) * t7990;
    let t12338 = -t12332 - t4499 + t4503 - t4506 - t4513 + t4539 + t4542 + t12333 + t12334 + t12335 + t12336 - t12337;
    (t12324, t12332, t12333, t12334, t12335, t12336, t12337, t12338)
}
