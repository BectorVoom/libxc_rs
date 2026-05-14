//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1010/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1010<F: Float>(t2101: F, t4790: F, t1985: F, t2179: F, t3578: F, t582: F, t2097: F, t1045: F, t16641: F, t61128: F, t16634: F, t142: F, t39430: F, t1526: F, t16644: F, t45751: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63586 = t2101 * t4790;
    let t63755 = t1985 * t2179;
    let t63855 = t582 * t3578;
    let t63863 = t2097 * t3578;
    let t64242 = t1985 * t1045;
    let t64621 = t61128 * t16641 / 9.0;
    let t64623 = 2.0 / 27.0 * t61128 * t16634;
    let t64631 = t39430 * t142;
    let t64642 = t1526 * t45751 * t16644;
    (t63586, t63755, t63855, t63863, t64242, t64621, t64623, t64631, t64642)
}
