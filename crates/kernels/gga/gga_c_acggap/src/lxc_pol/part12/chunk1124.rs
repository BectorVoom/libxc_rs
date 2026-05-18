//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1124/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1124<F: Float>(t1558: F, t30644: F, t4326: F, t7647: F, t1421: F, t1983: F, t30827: F, t7586: F, t1545: F, t31824: F, t1416: F, t1992: F, t30154: F) -> (F, F, F, F, F) {
    let t35979 = t30644 * t1558;
    let t35981 = t7647 * t4326;
    let t35985 = t30827 * t7586 * t1983 * t1421;
    let t35987 = t31824 * t1545;
    let t35991 = t30154 * t7586 * t1992 * t1416;
    (t35979, t35981, t35985, t35987, t35991)
}
