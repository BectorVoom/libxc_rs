//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1075/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1075<F: Float>(t1127: F, t688: F, t1689: F, t3771: F, t6813: F, t200: F, t3817: F, t2393: F, t4977: F, t222: F, t3722: F, t2379: F, t1095: F, t1113: F, t4939: F, t9523: F) -> (F, F, F, F, F, F, F) {
    let t65782 = t1127 * t688;
    let t66076 = t3771 * t6813 * t1689;
    let t66166 = t200 * t3817;
    let t66323 = t2393 * t4977;
    let t66382 = t3722 * t222;
    let t66383 = t2379 * t66382;
    let t66384 = t1095 * t1113;
    let t66397 = t9523 * t4939;
    (t65782, t66076, t66166, t66323, t66383, t66384, t66397)
}
