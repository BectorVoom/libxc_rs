//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 832/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk832<F: Float>(t6339: F, t681: F, t89: F, t1476: F, t7640: F, t375: F, t6343: F, t1486: F, t6323: F, t10631: F, t91: F, t26: F, t1487: F, t2399: F, t2347: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24986 = t681 * t6339;
    let t24987 = t89 * t24986;
    let t24989 = t7640 * t1476;
    let t24995 = t89 * t375 * t6343;
    let t25010 = t1486 * t681 * t6323;
    let t25026 = t91 * t10631;
    let t25027 = t25026 * t26;
    let t25035 = t1486 * t2399 * t1487;
    let t25036 = 2.0 / 9.0 * t25035;
    let t25037 = t1476 * t2347;
    (t24987, t24989, t24995, t25010, t25026, t25027, t25035, t25036, t25037)
}
