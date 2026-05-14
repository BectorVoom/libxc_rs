//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1289/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1289<F: Float>(t31307: F, t8392: F, t1882: F, t31217: F, t31186: F, t10157: F, t110010: F, t111320: F, t111322: F, t111324: F, t111389: F, t11593: F, t123890: F, t124029: F, t14127: F, t14163: F, t18525: F, t1901: F, t24789: F, t265: F, t28204: F, t31036: F, t3859: F, t3864: F, t446: F, t5073: F, t53923: F, t67996: F, t6852: F, t773: F, t97928: F) -> (F,) {
    let t124945 = t8392 * t31307;
    let t124958 = t1882 * t31217;
    let t124960 = t8392 * t31186;
    let t124970 = -t111320 - t111322 - t111324 + 4.0 / 9.0 * t1901 * t14163 * t124029 + 4.0 * t1901 * t110010 * t6852 * t3859 + 8.0 / 3.0 * t1901 * t67996 * t6852 * t3864 + 4.0 / 27.0 * t124945 - 2.0 / 9.0 * t1901 * t53923 * t28204 - 2.0 * t446 * t10157 * t773 * t31036 - 2.0 * t446 * t10157 * t265 * t123890 - 4.0 / 9.0 * t124958 - 2.0 / 27.0 * t124960 - 4.0 / 9.0 * t11593 * t24789 * t18525 - 4.0 / 3.0 * t1901 * t14127 * t97928 * t5073 - 8.0 / 27.0 * t111389;
    (t124970,)
}
