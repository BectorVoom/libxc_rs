//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1032/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1032<F: Float>(t3174: F, t6423: F, t68: F, t178: F, t404: F, t405: F, t4902: F, t2389: F, t5939: F, t918: F, t2099: F, t6516: F, t6519: F, t6525: F, t6527: F, t6508: F) -> (F, F, F, F, F, F) {
    let t19039 = t3174 * t68 * t6423;
    let t19055 = 0.14820648238345094262e-3 * t404 * t178 * t4902 * t405;
    let t19067 = t918 * t5939 * t2389;
    let t19070 = t6516 * t2099 * t6519;
    let t19073 = t6525 * t2099 * t6527;
    let t19076 = t918 * t2099 * t6508;
    (t19039, t19055, t19067, t19070, t19073, t19076)
}
