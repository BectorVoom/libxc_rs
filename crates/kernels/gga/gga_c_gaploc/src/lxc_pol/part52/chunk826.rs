//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 826/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk826<F: Float>(t13258: F, t484: F, t11481: F, t2321: F, t882: F, t1063: F, t11271: F, t6750: F, t2268: F, t3565: F, t6763: F, t13310: F) -> (F, F, F, F, F) {
    let t44623 = t484 * t13258;
    let t44624 = F::new(0.15808337019820083111e-2) * t44623;
    let t44626 = t882 * t11481 * t2321;
    let t44627 = F::new(0.11856252764865062333e-2) * t44626;
    let t44630 = F::new(0.85365019907028448797e-1) * t1063 * t11271 * t6750;
    let t44633 = F::new(0.42682509953514224398e0) * t2268 * t3565 * t6763;
    let t44634 = t484 * t13310;
    (t44624, t44627, t44630, t44633, t44634)
}
