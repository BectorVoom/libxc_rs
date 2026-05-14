//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 843/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk843<F: Float>(t5163: F, t649: F, t40999: F, t35960: F, t5166: F, t2079: F, t262: F, t570: F, t830: F, t2067: F, t2353: F, t26531: F, t551: F, t2068: F, t558: F, t2073: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t41000 = t649 * t5163;
    let t41001 = t40999 * t41000;
    let t41004 = t35960 * t649 * t5166;
    let t41021 = t2079 * t262 * t830 * t570;
    let t41024 = t26531 * t2067 * t2353;
    let t41027 = t830 * t551;
    let t41028 = t262 * t41027;
    let t41029 = t2068 * t41028;
    let t41031 = t830 * t558;
    let t41032 = t262 * t41031;
    let t41033 = t2073 * t41032;
    (t41000, t41001, t41004, t41021, t41024, t41027, t41028, t41029, t41031, t41032, t41033)
}
