//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3081/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3081<F: Float>(t56176: F, t56183: F, t43830: F, t43832: F, t44865: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F) -> F {
    let t56343 = F::cast_from(0.13170370370370370371e-1_f64) * t56176;
    let t56345 = F::cast_from(0.39511111111111111112e-1_f64) * t56183;
    let t56354 = t44865 - F::cast_from(0.35560000000000000001e0_f64) * t56151 + F::cast_from(0.88900000000000000002e-1_f64) * t56155 + F::cast_from(0.2667e0_f64) * t56159 + F::cast_from(0.29633333333333333334e-1_f64) * t56163 + F::cast_from(0.35560000000000000001e0_f64) * t56167 - F::cast_from(0.29633333333333333334e-1_f64) * t43830 + F::cast_from(0.98777777777777777781e-2_f64) * t43832 - F::cast_from(0.43901234567901234568e-1_f64) * t56174 - t56343 + F::cast_from(0.19755555555555555556e0_f64) * t56181 + t56345 - F::cast_from(0.59266666666666666668e-1_f64) * t56185 - F::cast_from(0.29633333333333333334e-1_f64) * t56187 - F::cast_from(0.88900000000000000002e-1_f64) * t56189 - F::cast_from(0.29633333333333333334e-1_f64) * t56194 - F::cast_from(0.29633333333333333334e-1_f64) * t56198 - F::cast_from(0.1778e0_f64) * t56203 - F::cast_from(0.9877777777777777778e-2_f64) * t56207 + F::cast_from(0.19755555555555555556e-1_f64) * t56209;
    t56354
}
