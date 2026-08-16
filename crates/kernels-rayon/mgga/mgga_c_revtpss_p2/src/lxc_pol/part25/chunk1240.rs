//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1240/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1240(t25266: f64, t2648: f64, t2681: f64, t7036: f64, t820: f64, t839: f64, t10878: f64, t7038: f64, t25260: f64, t843: f64, t2726: f64, t93021: f64, t93022: f64, t93026: f64, t93028: f64, t93031: f64, t93035: f64, t93037: f64, t93039: f64, t93041: f64, t93043: f64) -> f64 {
    let t93045 = t25266 * t2648;
    let t93048 = t820 * t7036 * t2681;
    let t93049 = t93048 * t839;
    let t93051 = t7038 * t10878;
    let t93054 = t820 * t25260 * t843;
    let t93055 = t93054 * t2726;
    let t93057 = -t93021 - 0.10289764348336736873e-1_f64 * t93022 + 0.15246000842785598468e-3_f64 * t93026 + 0.60023625365297631762e-2_f64 * t93028 - 0.34299214494455789577e-3_f64 * t93031 + 0.81312004494856525162e-3_f64 * t93035 + 0.25724410870841842184e-1_f64 * t93037 + 0.25724410870841842183e-2_f64 * t93039 - 0.42874018118069736972e-3_f64 * t93041 - 0.76230004213927992339e-4_f64 * t93043 + 0.60023625365297631762e-2_f64 * t93045 - 0.34013387707001991332e-1_f64 * t93049 - 0.42874018118069736972e-3_f64 * t93051 - 0.12004725073059526352e-1_f64 * t93055;
    t93057
}
