//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1233/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1233<F: Float>(t1092: F, t28991: F, t92917: F, t1662: F, t28182: F, t92693: F, t20345: F, t20349: F, t20573: F, t26955: F, t26960: F, t26966: F, t28153: F, t28204: F, t29094: F, t95581: F, t95585: F, t96781: F, t97083: F, t97089: F, t97093: F) -> (F, F) {
    let t100108 = t1092 * t92917 * t28991;
    let t100114 = t92693 * t1662 * t28182;
    let t100128 = t96781 - F::new(0.41270617283950617283e-2) * t95581 + F::new(0.92754700520833333334e-4) * t28204 * t28153 - F::new(0.23214722222222222222e-2) * t100108 - F::new(0.46336805555555555556e-3) * t26960 * t97093 * t20573 - F::new(0.23168402777777777778e-3) * t26960 * t100114 - F::new(0.46336805555555555556e-3) * t26960 * t97083 * t20345 - F::new(0.30918233506944444445e-4) * t26955 * t100114 + F::new(0.30891203703703703704e-3) * t26960 * t97089 * t20349 + F::new(0.18534722222222222222e-2) * t26966 * t29094 + F::new(0.46429444444444444444e-2) * t95585;
    (t100108, t100128)
}
