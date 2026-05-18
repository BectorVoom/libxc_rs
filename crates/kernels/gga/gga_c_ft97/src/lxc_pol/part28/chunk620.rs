//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 620/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk620<F: Float>(t25969: F, t446: F, t25919: F, t7824: F, t1564: F, t25924: F, t25929: F, t7793: F, t22953: F, t379: F, t6495: F, t22952: F) -> (F, F, F, F, F, F, F, F) {
    let t25970 = t446 * t25969;
    let t25972 = t7824 * t25919;
    let t25973 = t446 * t25972;
    let t25975 = t1564 * t25924;
    let t25976 = t446 * t25975;
    let t25978 = t7793 * t25929;
    let t25979 = t446 * t25978;
    let t25982 = t22953 * t6495 * t379;
    let t25983 = t22952 * t25982;
    (t25970, t25972, t25973, t25975, t25976, t25978, t25979, t25983)
}
