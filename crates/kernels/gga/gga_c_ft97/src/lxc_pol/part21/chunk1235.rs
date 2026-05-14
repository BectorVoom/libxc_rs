//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1235/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1235<F: Float>(t4710: F, t58: F, t22591: F, t554: F, t538: F, t1701: F, t22652: F, t4698: F, t2035: F, t4702: F, t5790: F, t22632: F, t23732: F, t30063: F, t1013: F, t104915: F) -> (F, F, F, F, F, F, F) {
    let t118837 = t58 * t4710;
    let t118839 = t22591 * t118837 * t554;
    let t118843 = t22591 * t118837 * t538;
    let t118847 = t1701 * t22652 * t4710;
    let t118852 = t1701 * t22652 * t4698;
    let t118856 = t2035 * t5790 * t4702;
    let t118869 = t23732 * t22632 * t30063;
    let t118876 = t22591 * t104915 * t1013;
    (t118839, t118843, t118847, t118852, t118856, t118869, t118876)
}
