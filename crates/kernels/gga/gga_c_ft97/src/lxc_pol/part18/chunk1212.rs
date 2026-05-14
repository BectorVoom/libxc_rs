//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1212/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1212<F: Float>(t1286: F, t25552: F, t376: F, t6418: F, t94032: F, t458: F, t6413: F, t5504: F, t11176: F, t1285: F, t25579: F, t497: F, t6454: F, t101573: F, t108: F, t11618: F, t1564: F, t1570: F, t22907: F, t22910: F, t22935: F, t25558: F, t25609: F, t25849: F, t25863: F, t28: F, t3188: F, t3204: F, t369: F, t379: F, t38921: F, t5495: F, t5501: F, t5502: F, t91480: F, t93888: F) -> (F,) {
    let t101943 = t1286 * t376 * t25552 / 9.0;
    let t101949 = t94032 * t6418 / 27.0;
    let t101957 = t6413 * t458;
    let t101959 = t101957 * t5504 / 27.0;
    let t101961 = t1285 * t11176 * t25579;
    let t101975 = t6454 * t497;
    let t101982 = -t101943 - 4.0 * t5501 * t38921 * t5502 * t11618 + t101949 + t5495 * t25849 / 3.0 + t1286 * t28 * t369 * t101573 * t108 / 6.0 + t101959 + 11.0 / 27.0 * t101961 + 2.0 / 9.0 * t93888 + 2.0 / 9.0 * t5501 * t22907 * t91480 * t3204 + 2.0 / 9.0 * t25558 * t22910 + 2.0 / 9.0 * t5501 * t25609 * t497 * t1570 * t3188 - t5501 * t1564 * t101975 * t379 / 9.0 - t22935 * t25863 / 9.0;
    (t101982,)
}
