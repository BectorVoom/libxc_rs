//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1135/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1135<F: Float>(t23448: F, t8392: F, t582: F, t5929: F, t23452: F, t1882: F, t23529: F, t2101: F, t5935: F, t23457: F, t23480: F, t5842: F, t604: F, t23986: F, t160: F, t23884: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95762 = t8392 * t23448;
    let t95767 = t582 * t5929;
    let t95771 = t8392 * t23452;
    let t95776 = t1882 * t23529;
    let t95789 = t2101 * t5935;
    let t95797 = t8392 * t23457;
    let t95799 = t1882 * t23480;
    let t95813 = t604 * t5842;
    let t95820 = t1882 * t23986;
    let t95822 = t160 * t23884;
    (t95762, t95767, t95771, t95776, t95789, t95797, t95799, t95813, t95820, t95822)
}
