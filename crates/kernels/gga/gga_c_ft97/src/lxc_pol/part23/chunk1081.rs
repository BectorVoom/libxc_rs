//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1081/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1081<F: Float>(t18969: F, t61128: F, t18962: F, t294: F, t43194: F, t1526: F, t4037: F, t9483: F, t18972: F, t52679: F, t4052: F, t2252: F, t342: F, t5202: F, t18982: F, t630: F) -> (F, F, F, F, F, F, F, F) {
    let t72910 = t61128 * t18969 / 9.0;
    let t72912 = 2.0 / 27.0 * t61128 * t18962;
    let t72944 = t43194 * t294;
    let t72950 = t1526 * t9483 * t4037 / 18.0;
    let t72952 = t1526 * t52679 * t18972;
    let t72962 = t1526 * t9483 * t4052 / 18.0;
    let t72977 = t342 * t2252 * t5202;
    let t72981 = t342 * t630 * t18982 / 6.0;
    (t72910, t72912, t72944, t72950, t72952, t72962, t72977, t72981)
}
