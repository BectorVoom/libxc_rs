//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1422/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1422(t34772: f64, t34776: f64, t34797: f64, t34802: f64, t37072: f64, t37073: f64, t37074: f64, t37075: f64, t37076: f64, t37077: f64, t37080: f64, t34820: f64, t37082: f64, t37083: f64, t37084: f64, t37086: f64, t37087: f64, t37088: f64, t37089: f64, t37090: f64, t37091: f64, t37092: f64) -> (f64, f64) {
    let t38615 = 0.19336854506021130164e-7_f64 * t34772 - 0.52389984474979915325e-9_f64 * t34776 - t37072 - t37073 - t37074 + t37075 + t37076 + t37077 + 0.29465683056794103108e-8_f64 * t34797 - 0.98332751566569010432e-8_f64 * t34802 + t37080;
    let t38617 = t37082 - t37083 + t37084 - 0.20912781366153999614e-9_f64 * t34820 - t37086 + t37087 - t37088 + t37089 - t37090 + t37091 + t37092;
    (t38615, t38617)
}
