//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1050/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1050<F: Float>(t11484: F, t1835: F, t1691: F, t1040: F, t34382: F, t11387: F, t2993: F, t8793: F, t11320: F, t1700: F, t633: F, t3708: F, t9071: F, t9256: F, t1416: F, t3116: F, t9180: F) -> (F, F, F, F, F, F, F) {
    let t35003 = t11484 * t1835;
    let t35005 = t11484 * t1691;
    let t35007 = t34382 * t1040;
    let t35010 = t2993 * t11387 * t8793;
    let t35013 = t633 * t11320 * t1700;
    let t35016 = t9071 * t3708 * t9256;
    let t35019 = t9180 * t1416 * t3116;
    (t35003, t35005, t35007, t35010, t35013, t35016, t35019)
}
