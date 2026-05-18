//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1150/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1150<F: Float>(t560: F, t922: F, t811: F, t839: F, t694: F, t9114: F, t10761: F, t15026: F, t1680: F, t2249: F, t2254: F, t24605: F, t24623: F, t32257: F, t32264: F, t33335: F, t4818: F, t4822: F, t5399: F, t567: F, t643: F, t7297: F, t8034: F, t8356: F, t8372: F, t9089: F, t9096: F, t9460: F) -> (F, F, F, F) {
    let t36577 = t560 * t922;
    let t36611 = t560 * t811;
    let t36647 = t560 * t839;
    let t36684 = F::new(6.0) * t694 * t9114;
    let t36685 = -F::new(6.0) * t10761 * t7297 * t9089 - t15026 * t567 * t643 - t1680 * t567 * t8356 - F::new(2.0) * t2249 * t5399 * t567 + F::new(3.0) * t2254 * t33335 * t567 + F::new(4.0) * t24605 * t9096 * t9460 + F::new(6.0) * t24623 * t7297 * t9460 + F::new(12.0) * t4818 * t8034 * t8372 + F::new(6.0) * t4822 * t8034 * t8372 + F::new(2.0) * t32257 + F::new(6.0) * t32264 - t36684;
    (t36577, t36611, t36647, t36685)
}
