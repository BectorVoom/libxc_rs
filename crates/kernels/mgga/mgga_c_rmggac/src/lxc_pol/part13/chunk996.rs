//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 996/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk996<F: Float>(t34724: F, t8646: F, t34735: F, t8650: F, t118: F, t128: F, t1986: F, t1994: F, t5735: F, t30137: F, t681: F, t2034: F, t30174: F) -> (F, F, F, F, F) {
    let t41836 = t34724 * t8646;
    let t41838 = t34735 * t8650;
    let t41846 = t1994 * t1986 * t118 * t128 * t5735;
    let t41848 = t30137 * t681;
    let t41850 = t30174 * t2034;
    (t41836, t41838, t41846, t41848, t41850)
}
