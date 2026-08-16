//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1110/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1110(t15882: f64, t333: f64, t326: f64, t69418: f64, t69420: f64, t69424: f64, t72038: f64, t78069: f64, t78072: f64, t78073: f64, t78077: f64, t78078: f64, t78079: f64, t78083: f64, t78091: f64, t78094: f64) -> (f64, f64) {
    let t80478 = t15882 * t333;
    let t80482 = -t78069 + t78072 + t78073 + t78077 + t78078 - t78079 - t72038 + t78083 - 0.59871208509319042821e-1_f64 * t326 * t80478 - t78091 + t78094 - t69418 + t69420 - 0.8283415761659696377e-1_f64 * t69424;
    (t80478, t80482)
}
