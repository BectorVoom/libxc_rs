//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3063/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3063<F: Float>(t43830: F, t43832: F, t43995: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56176: F, t56181: F, t56184: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F) -> F {
    let t56211 = t43995 - F::cast_from(0.22249999999999999999e0_f64) * t56151 + F::cast_from(0.55625000000000000001e-1_f64) * t56155 + F::cast_from(0.166875e0_f64) * t56159 + F::cast_from(0.18541666666666666667e-1_f64) * t56163 + F::cast_from(0.2225e0_f64) * t56167 - F::cast_from(0.18541666666666666667e-1_f64) * t43830 + F::cast_from(0.61805555555555555556e-2_f64) * t43832 - F::cast_from(0.27469135802469135803e-1_f64) * t56174 - F::cast_from(0.82407407407407407408e-2_f64) * t56176 + F::cast_from(0.12361111111111111111e0_f64) * t56181 + t56184 - F::cast_from(0.37083333333333333333e-1_f64) * t56185 - F::cast_from(0.18541666666666666667e-1_f64) * t56187 - F::cast_from(0.55625e-1_f64) * t56189 - F::cast_from(0.18541666666666666666e-1_f64) * t56194 - F::cast_from(0.18541666666666666666e-1_f64) * t56198 - F::cast_from(0.11125e0_f64) * t56203 - F::cast_from(0.61805555555555555555e-2_f64) * t56207 + F::cast_from(0.12361111111111111111e-1_f64) * t56209;
    t56211
}
