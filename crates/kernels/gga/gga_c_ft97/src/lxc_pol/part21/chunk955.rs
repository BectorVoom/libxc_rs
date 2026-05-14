//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 955/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk955<F: Float>(t1307: F, t4551: F, t1852: F, t452: F, t110: F, t29721: F, t8411: F, t4458: F, t5717: F, t1909: F, t4623: F, t6564: F, t942: F, t23265: F, t4606: F, t8557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29986 = t1307 * t4551;
    let t29988 = t452 * t1852 * t29986;
    let t29992 = t8411 * t110 * t29721;
    let t29995 = t5717 * t4458;
    let t29996 = t1909 * t29995;
    let t30001 = t452 * t4623 * t1307;
    let t30005 = t452 * t6564 * t942;
    let t30008 = t23265 * t4606;
    let t30009 = t8557 * t30008;
    (t29986, t29988, t29992, t29995, t29996, t30001, t30005, t30008, t30009)
}
