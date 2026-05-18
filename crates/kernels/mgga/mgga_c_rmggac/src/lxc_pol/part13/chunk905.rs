//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 905/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk905<F: Float>(t36978: F, t5169: F, t656: F, t34738: F, t5260: F, t36471: F, t5263: F, t1550: F, t2060: F, t29892: F, t27044: F, t903: F) -> (F, F, F, F, F) {
    let t40012 = t36978 * t656 * t5169;
    let t40015 = t34738 * t656 * t5260;
    let t40018 = t36471 * t656 * t5263;
    let t40021 = t1550 * t2060 * t29892;
    let t40024 = t903 * t2060 * t27044;
    (t40012, t40015, t40018, t40021, t40024)
}
