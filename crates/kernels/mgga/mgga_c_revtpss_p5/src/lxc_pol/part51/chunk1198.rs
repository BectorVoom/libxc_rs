//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1198/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1198<F: Float>(t121468: F, t121470: F, t127428: F, t127434: F, t127437: F, t127477: F, t127508: F, t1456: F, t1458: F, t1914: F, t2038: F, t28283: F, t32378: F, t34015: F, t5790: F, t7319: F, t7337: F, t7940: F, t7956: F, t8617: F) -> F {
    let t127511 = F::new(2.0) * t7940 * t7337 + F::new(2.0) * t127428 + F::new(2.0) * t7319 * t7956 + F::new(2.0) * t2038 * t28283 + t121468 + t121470 + t127434 + t5790 * t8617 + t1914 * t32378 + t127437 + t1456 * t34015 + t1458 * (t127477 + t127508);
    t127511
}
