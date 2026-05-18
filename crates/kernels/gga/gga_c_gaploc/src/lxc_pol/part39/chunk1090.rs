//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1090/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1090<F: Float>(t11977: F, t2268: F, t6763: F, t1063: F, t6750: F, t42857: F, t42863: F, t42866: F, t42867: F, t42868: F, t42869: F, t42870: F, t42871: F, t42872: F) -> F {
    let t47047 = t2268 * t11977 * t6763;
    let t47050 = t1063 * t11977 * t6750;
    let t47052 = -F::new(0.19918504644973304719e0) * t47047 + t42857 + t42863 + t42866 - t42867 + t42868 + F::new(0.85365019907028448797e-1) * t47050 - t42869 - t42870 - t42871 + t42872;
    t47052
}
