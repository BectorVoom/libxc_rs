//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2511/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2511<F: Float>(t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t43895: F, t50976: F, t50978: F, t50987: F, t50990: F, t50994: F) -> F {
    let t50996 = t43895 - F::cast_from(0.8585111111111111111e-1_f64) * t50976 - F::cast_from(0.73586666666666666668e-1_f64) * t50978 + F::cast_from(0.40256666666666666667e0_f64) * t43780 + F::cast_from(0.80513333333333333335e0_f64) * t43782 + F::cast_from(0.40256666666666666668e0_f64) * t43784 - F::cast_from(0.60385000000000000002e0_f64) * t43786 - F::cast_from(0.10064166666666666667e0_f64) * t43788 - F::cast_from(0.93932222222222222223e0_f64) * t43816 + F::cast_from(0.11038e0_f64) * t50987 + F::cast_from(0.44152e0_f64) * t50990 - F::cast_from(0.36231e1_f64) * t50994;
    t50996
}
