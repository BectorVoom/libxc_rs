//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1185/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1185(t1121: f64, t6491: f64, t3438: f64, t5175: f64, t15068: f64, t5091: f64, t1195: f64, t6731: f64, t382: f64, t19789: f64, t5176: f64, t1166: f64, t6705: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19856 = t6491 * t1121;
    let t19857 = t3438 * t19856;
    let t19858 = t5175 * t19857;
    let t19860 = t15068 * t5091;
    let t19862 = t1195 * t6731;
    let t19863 = t382 * t19862;
    let t19865 = t5176 * t19789;
    let t19866 = t5175 * t19865;
    let t19868 = t1166 * t6705;
    (t19856, t19858, t19860, t19863, t19866, t19868)
}
