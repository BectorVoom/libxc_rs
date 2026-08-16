//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 871/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk871<F: Float>(t30169: F, t601: F, t3646: F, t597: F, t606: F, t1979: F, t980: F, t1994: F, t7736: F, t993: F, t12935: F, t2067: F) -> (F, F, F, F, F, F, F) {
    let t30191 = t30169 * t601;
    let t30193 = t3646 * t597;
    let t30194 = t30193 * t606;
    let t30196 = t980 * t1979;
    let t30197 = t30196 * t1994;
    let t30199 = t7736 * t993;
    let t30209 = t12935 * t2067;
    (t30191, t30193, t30194, t30196, t30197, t30199, t30209)
}
