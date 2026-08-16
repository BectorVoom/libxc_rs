//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 761/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk761<F: Float>(t7491: F, t7927: F, t20: F, t2018: F, t2021: F, t4710: F, t261: F, t7581: F, t2013: F, t7338: F, t20963: F, t82: F) -> (F, F, F, F, F, F) {
    let t35698 = t7491 * t7927;
    let t35702 = t4710 * t20 * t2018 * t2021;
    let t35704 = t261 * t7581;
    let t35705 = t35704 * t2013;
    let t35707 = t7491 * t7338;
    let t35709 = t20963 * t82;
    (t35698, t35702, t35704, t35705, t35707, t35709)
}
