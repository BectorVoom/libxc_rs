//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 840/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk840<F: Float>(t25955: F, t363: F, t1564: F, t446: F, t18: F, t5691: F, t3281: F, t432: F, t6469: F, t8411: F, t25933: F, t7824: F, t25919: F, t25924: F, t25929: F, t7793: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25956 = t25955 * t363;
    let t25957 = t1564 * t25956;
    let t25958 = t446 * t25957;
    let t25960 = t5691 * t18;
    let t25961 = t1564 * t25960;
    let t25962 = t3281 * t25961;
    let t25965 = t8411 * t6469 * t432;
    let t25966 = t446 * t25965;
    let t25969 = t7824 * t25933;
    let t25970 = t446 * t25969;
    let t25972 = t7824 * t25919;
    let t25973 = t446 * t25972;
    let t25975 = t1564 * t25924;
    let t25976 = t446 * t25975;
    let t25978 = t7793 * t25929;
    (t25957, t25958, t25961, t25962, t25965, t25966, t25969, t25970, t25972, t25973, t25975, t25976, t25978)
}
