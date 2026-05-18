//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1165/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1165<F: Float>(t294: F, t9570: F, t2252: F, t2644: F, t342: F, t10231: F, t630: F, t784: F, t8639: F, t10236: F, t10388: F, t10410: F, t10422: F, t10426: F, t10432: F, t13605: F, t1526: F, t231: F, t2320: F, t2639: F, t343: F, t3806: F, t8608: F, t9571: F) -> F {
    let t44700 = t294 * t9570;
    let t44709 = t342 * t2252 * t2644;
    let t44712 = t342 * t630 * t10231;
    let t44716 = F::new(5.0) / F::new(54.0) * t342 * t8639 * t784;
    let t44717 = t10236 - t342 * t343 * t231 * t10388 / F::new(4.0) - t1526 * t2320 * t10422 / F::new(4.0) - t1526 * t2320 * t2639 * t8608 / F::new(12.0) - t1526 * t3806 * t10410 / F::new(3.0) - F::new(7.0) / F::new(27.0) * t1526 * t13605 * t44700 * t9571 - t1526 * t2320 * t10426 / F::new(4.0) + t10432 + t44709 / F::new(6.0) - t44712 / F::new(4.0) - t44716;
    t44717
}
