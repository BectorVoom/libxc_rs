//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 837/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk837<F: Float>(t1800: F, t22975: F, t1317: F, t28: F, t376: F, t5680: F, t1580: F, t5691: F, t1564: F, t446: F, t1307: F, t1557: F) -> (F, F, F, F, F, F) {
    let t22976 = t1800 * t22975;
    let t22978 = t1317 * t28 * t22976;
    let t22980 = t1317 * t376 * t5680;
    let t22982 = t5691 * t1580;
    let t22983 = t1564 * t22982;
    let t22984 = t446 * t22983;
    let t22986 = t1307 * t1557;
    (t22976, t22978, t22980, t22983, t22984, t22986)
}
