//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1244/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1244<F: Float>(t1851: F, t6557: F, t1326: F, t1587: F, t6475: F, t8232: F, t1882: F, t26480: F, t26211: F, t46862: F, t100315: F, t11485: F, t11490: F, t11812: F, t11863: F, t1651: F, t1876: F, t1901: F, t22993: F, t23249: F, t26145: F, t26240: F, t3113: F, t379: F, t446: F, t447: F, t47831: F, t5718: F, t6538: F, t6564: F, t8557: F, t91743: F, t91745: F) -> (F,) {
    let t103068 = t1851 * t6557;
    let t103073 = t1587 * t1326;
    let t103077 = t8232 * t6475;
    let t103082 = 2.0 / 9.0 * t1882 * t26480;
    let t103083 = t46862 * t26211;
    let t103085 = -t446 * t447 * t6564 * t1651 / 9.0 - 2.0 / 9.0 * t1901 * t8557 * t26145 * t379 - 2.0 / 9.0 * t1901 * t8557 * t26240 * t379 - t1901 * t8557 * t6538 * t1651 / 9.0 + 2.0 / 9.0 * t1901 * t47831 * t5718 - 4.0 / 3.0 * t1901 * t11490 * t23249 * t11485 - 4.0 / 9.0 * t1901 * t11863 * t100315 - 2.0 / 9.0 * t1901 * t8557 * t22993 * t3113 - 4.0 / 3.0 * t1901 * t11490 * t103068 * t1876 - 4.0 / 3.0 * t1901 * t103073 * t11812 - 4.0 / 27.0 * t103077 - 2.0 / 9.0 * t91743 - t91745 / 9.0 + t103082 - 22.0 / 27.0 * t103083;
    (t103085,)
}
