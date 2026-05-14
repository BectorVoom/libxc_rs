//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1115/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1115<F: Float>(t2726: F, t93054: F, t93021: F, t93022: F, t93026: F, t93028: F, t93031: F, t93035: F, t93037: F, t93039: F, t93041: F, t93043: F, t93045: F, t93049: F, t93051: F, t10841: F, t25245: F) -> (F, F) {
    let t93055 = t93054 * t2726;
    let t93057 = -t93021 - 0.10289764348336736873e-1 * t93022 + 0.15246000842785598468e-3 * t93026 + 0.60023625365297631762e-2 * t93028 - 0.34299214494455789577e-3 * t93031 + 0.81312004494856525162e-3 * t93035 + 0.25724410870841842184e-1 * t93037 + 0.25724410870841842183e-2 * t93039 - 0.42874018118069736972e-3 * t93041 - 0.76230004213927992339e-4 * t93043 + 0.60023625365297631762e-2 * t93045 - 0.34013387707001991332e-1 * t93049 - 0.42874018118069736972e-3 * t93051 - 0.12004725073059526352e-1 * t93055;
    let t93058 = t25245 * t10841;
    (t93057, t93058)
}
