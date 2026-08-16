//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2551/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2551<F: Float>(t50948: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t43942: F, t50937: F, t50940: F, t50946: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50994: F, t51000: F, t51004: F) -> F {
    let t51707 = F::cast_from(0.24722222222222222222e-1_f64) * t50948;
    let t51723 = F::cast_from(0.166875e0_f64) * t50937 + F::cast_from(0.18541666666666666667e-1_f64) * t50940 + F::cast_from(0.2225e0_f64) * t50946 + t51707 + F::cast_from(0.12361111111111111111e-1_f64) * t50950 + F::cast_from(0.61805555555555555556e-2_f64) * t50952 + F::cast_from(0.37083333333333333334e-1_f64) * t50954 - F::cast_from(0.18541666666666666666e-1_f64) * t50957 - F::cast_from(0.18541666666666666666e-1_f64) * t50961 - F::cast_from(0.11125e0_f64) * t50966 + t43942 + F::cast_from(0.12361111111111111111e-1_f64) * t43780 + F::cast_from(0.24722222222222222222e-1_f64) * t43782 + F::cast_from(0.12361111111111111111e-1_f64) * t43784 - F::cast_from(0.18541666666666666667e-1_f64) * t43786 - F::cast_from(0.30902777777777777778e-2_f64) * t43788 - F::cast_from(0.28842592592592592593e-1_f64) * t43816 - F::cast_from(0.11125e0_f64) * t50994 + F::cast_from(0.166875e0_f64) * t51000 + F::cast_from(0.30902777777777777778e-1_f64) * t51004;
    t51723
}
