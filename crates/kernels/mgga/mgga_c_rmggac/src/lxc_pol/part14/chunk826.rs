//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 826/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk826<F: Float>(t34738: F, t5260: F, t656: F, t36471: F, t5263: F, t1550: F, t2060: F, t29892: F, t27044: F, t903: F, t27120: F, t739: F, t7577: F, t2001: F, t2281: F, t305: F, t321: F) -> (F, F, F, F, F, F) {
    let t40015 = t34738 * t656 * t5260;
    let t40018 = t36471 * t656 * t5263;
    let t40021 = t1550 * t2060 * t29892;
    let t40024 = t903 * t2060 * t27044;
    let t40027 = t739 * t7577 * t27120;
    let t40031 = t2001 * t305 * t2281 * t321;
    (t40015, t40018, t40021, t40024, t40027, t40031)
}
