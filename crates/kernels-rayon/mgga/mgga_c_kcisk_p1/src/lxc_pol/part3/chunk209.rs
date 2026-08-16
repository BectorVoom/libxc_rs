//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 209/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk209(t135: f64, t60: f64, t4: f64, t68: f64, t85: f64, t73: f64, t2: f64, t41: f64, t74: f64, t818: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t854 = t135 * t135;
    let t855 = 1.0_f64 / t854;
    let t856 = t60 * t855;
    let t857 = t68 * t4;
    let t861 = t85 * t85;
    let t862 = 1.0_f64 / t861;
    let t863 = t73 * t862;
    let t866 = 1.0_f64 / t74 * t41 * t2;
    let t867 = t866 * t818;
    let t869 = t68 * t818;
    let t871 = f64::sqrt(t71);
    let t873 = t871 * t41 * t2;
    let t874 = t873 * t818;
    (t854, t855, t856, t857, t861, t862, t863, t866, t867, t869, t873, t874)
}
