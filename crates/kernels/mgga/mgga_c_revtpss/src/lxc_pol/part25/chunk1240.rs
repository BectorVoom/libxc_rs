//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1240/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1240<F: Float>(t25266: F, t2648: F, t2681: F, t7036: F, t820: F, t839: F, t10878: F, t7038: F, t25260: F, t843: F, t2726: F, t93021: F, t93022: F, t93026: F, t93028: F, t93031: F, t93035: F, t93037: F, t93039: F, t93041: F, t93043: F) -> F {
    let t93045 = t25266 * t2648;
    let t93048 = t820 * t7036 * t2681;
    let t93049 = t93048 * t839;
    let t93051 = t7038 * t10878;
    let t93054 = t820 * t25260 * t843;
    let t93055 = t93054 * t2726;
    let t93057 = -t93021 - F::new(0.10289764348336736873e-1) * t93022 + F::new(0.15246000842785598468e-3) * t93026 + F::new(0.60023625365297631762e-2) * t93028 - F::new(0.34299214494455789577e-3) * t93031 + F::new(0.81312004494856525162e-3) * t93035 + F::new(0.25724410870841842184e-1) * t93037 + F::new(0.25724410870841842183e-2) * t93039 - F::new(0.42874018118069736972e-3) * t93041 - F::new(0.76230004213927992339e-4) * t93043 + F::new(0.60023625365297631762e-2) * t93045 - F::new(0.34013387707001991332e-1) * t93049 - F::new(0.42874018118069736972e-3) * t93051 - F::new(0.12004725073059526352e-1) * t93055;
    t93057
}
