//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1296/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1296<F: Float>(t1165: F, t3361: F, t4267: F, t4718: F, t1173: F, t14346: F, t1552: F, t18655: F, t18657: F, t18660: F, t18672: F, t18683: F, t18686: F, t20595: F, t301: F, t3403: F, t4298: F, t4463: F, t4752: F, t530: F, t5621: F, t6151: F) -> F {
    let t24084 = t3361 * t1165 * t4267 * t4718;
    let t24104 = -F::cast_from(0.40015750243531754508e-2_f64) * t18655 - F::cast_from(0.17149607247227894789e-2_f64) * t18657 + F::cast_from(0.42874018118069736972e-3_f64) * t14346 + F::cast_from(0.34299214494455789578e-2_f64) * t18660 + F::cast_from(0.51448821741683684367e-2_f64) * t18672 - F::cast_from(0.51448821741683684367e-2_f64) * t18683 - F::cast_from(0.68598428988911579156e-2_f64) * t24084 - F::cast_from(0.34299214494455789578e-1_f64) * t4463 * t1165 * t4267 * t4752 + F::cast_from(0.16006300097412701803e0_f64) * t18686 - F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t1165 * t4298 * t5621 - F::cast_from(0.17149607247227894789e-1_f64) * t3403 * t1165 * t530 * t20595 - F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t1165 * t1552 * t6151 * t301;
    t24104
}
