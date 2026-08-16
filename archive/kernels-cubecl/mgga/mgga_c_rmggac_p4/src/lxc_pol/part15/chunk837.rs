//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 837/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk837<F: Float>(t1632: F, t2084: F, t7599: F, t41301: F, t8750: F, t41307: F, t7603: F, t25607: F, t27: F, t3851: F, t39692: F, t3826: F) -> (F, F, F, F, F, F, F) {
    let t41313 = t2084 * t1632;
    let t41314 = t7599 * t41313;
    let t41315 = F::cast_from(0.72732431077987577946e-1_f64) * t41314;
    let t41319 = t8750 * t41301;
    let t41320 = F::cast_from(0.2419210303588817044e-2_f64) * t41319;
    let t41323 = t7603 * t41307;
    let t41324 = F::cast_from(0.33868944250243438616e-2_f64) * t41323;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    let t41338 = t3851 * t39692;
    let t41340 = t3826 * t39692;
    (t41315, t41320, t41324, t41327, t41329, t41338, t41340)
}
