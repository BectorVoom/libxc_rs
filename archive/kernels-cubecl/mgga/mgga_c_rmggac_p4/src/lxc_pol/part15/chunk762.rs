//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 762/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk762<F: Float>(t35716: F, t302: F, t7350: F, t7349: F, t7353: F, t31: F, t35214: F, t7351: F, t35604: F, t7338: F, t7345: F, t7341: F) -> (F, F, F, F, F, F) {
    let t35717 = F::cast_from(0.13010691197123848594e-3_f64) * t35716;
    let t35718 = t7350 * t302;
    let t35720 = t7349 * t35718 * t7353;
    let t35724 = t7349 * t7351 * t35214 * t31;
    let t35728 = t7349 * t7351 * t35604 * t31;
    let t35729 = F::cast_from(0.65053455985619242968e-4_f64) * t35728;
    let t35742 = t7345 * t7338;
    let t35744 = t7345 * t7341;
    (t35717, t35720, t35724, t35729, t35742, t35744)
}
