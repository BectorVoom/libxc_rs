//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1168/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1168<F: Float>(t2399: F, t6909: F, t89: F, t1882: F, t28239: F, t28422: F, t28334: F, t28269: F, t1456: F, t9895: F, t10007: F, t108016: F, t108033: F, t108206: F, t13757: F, t14075: F, t14103: F, t14163: F, t14182: F, t14196: F, t14200: F, t14213: F, t1901: F, t2409: F, t24569: F, t2568: F, t2569: F, t2574: F, t28386: F, t3281: F, t3746: F, t446: F, t6194: F, t6837: F, t6921: F, t724: F, t729: F) -> (F,) {
    let t111290 = t89 * t2399 * t6909;
    let t111310 = 4.0 / 9.0 * t1882 * t28239;
    let t111320 = 4.0 / 9.0 * t1882 * t28422;
    let t111322 = 2.0 / 9.0 * t1882 * t28334;
    let t111324 = 2.0 / 9.0 * t1882 * t28269;
    let t111330 = t9895 * t1456;
    let t111338 = 4.0 / 27.0 * t111290 - 4.0 / 9.0 * t1901 * t14163 * t108206 - t1901 * t10007 * t24569 * t14103 / 9.0 - 4.0 / 9.0 * t1901 * t14196 * t108016 + 4.0 / 27.0 * t1901 * t14200 * t108033 + 2.0 / 9.0 * t1901 * t10007 * t6921 * t2409 - t111310 + 2.0 / 3.0 * t446 * t2574 * t1456 * t14213 + 4.0 / 9.0 * t3281 * t724 * t6194 * t3746 - t111320 - t111322 - t111324 - 2.0 / 3.0 * t446 * t729 * t2568 * t6837 * t2569 - 4.0 / 9.0 * t1901 * t111330 * t13757 - 2.0 / 9.0 * t1901 * t14182 * t28386 * t14075;
    (t111338,)
}
