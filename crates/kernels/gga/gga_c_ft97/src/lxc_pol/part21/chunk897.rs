//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 897/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk897<F: Float>(t27034: F, t363: F, t1969: F, t446: F, t558: F, t6630: F, t9432: F, t18: F, t5916: F, t3281: F, t26768: F, t586: F, t1369: F, t28: F, t1882: F, t6674: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27035 = t27034 * t363;
    let t27036 = t1969 * t27035;
    let t27037 = t446 * t27036;
    let t27040 = t9432 * t6630 * t558;
    let t27041 = t446 * t27040;
    let t27043 = t5916 * t18;
    let t27044 = t1969 * t27043;
    let t27045 = t3281 * t27044;
    let t27047 = t586 * t26768;
    let t27049 = t1369 * t28 * t27047;
    let t27051 = t1882 * t6674;
    (t27036, t27037, t27040, t27041, t27044, t27045, t27047, t27049, t27051)
}
