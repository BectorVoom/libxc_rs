//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 839/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk839<F: Float>(t22993: F, t363: F, t1564: F, t446: F, t473: F, t5617: F, t469: F, t28: F, t5665: F, t1307: F, t1808: F, t8345: F, t91: F, t26: F, t1767: F, t376: F, t5667: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22994 = t22993 * t363;
    let t22995 = t1564 * t22994;
    let t22996 = t446 * t22995;
    let t22998 = t5617 * t473;
    let t22999 = t469 * t22998;
    let t23001 = t5665 * t28 * t22999;
    let t23003 = t1307 * t1808;
    let t23004 = t469 * t23003;
    let t23006 = t5665 * t28 * t23004;
    let t23008 = t91 * t8345;
    let t23009 = t23008 * t26;
    let t23010 = t1307 * t1767;
    let t23011 = t469 * t23010;
    let t23013 = t23009 * t28 * t23011;
    let t23016 = t5665 * t376 * t5667;
    (t22995, t22996, t22999, t23001, t23004, t23006, t23008, t23009, t23011, t23013, t23016)
}
