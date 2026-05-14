//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 783/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk783<F: Float>(t13846: F, t1941: F, t13877: F, t2018: F, t5617: F, t807: F, t241: F, t25981: F, t820: F, t5677: F, t26028: F, t5697: F, t5701: F, t5706: F, t5614: F, t7271: F) -> (F, F, F, F, F, F, F) {
    let t27932 = t1941 * t13846;
    let t27933 = t27932 * t13877;
    let t27936 = t2018 * t5617;
    let t27937 = t807 * t27936;
    let t27940 = t820 * t25981 * t241;
    let t27941 = t27940 * t5677;
    let t27943 = t26028 * t5697;
    let t27945 = t26028 * t5701;
    let t27947 = t26028 * t5706;
    let t27949 = t7271 * t5614;
    (t27933, t27937, t27941, t27943, t27945, t27947, t27949)
}
