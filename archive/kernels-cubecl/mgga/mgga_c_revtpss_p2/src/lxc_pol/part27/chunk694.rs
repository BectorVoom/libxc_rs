//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 694/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk694<F: Float>(t651: F, t7003: F, t30: F, t775: F, t1949: F, t212: F, t780: F, t689: F, t1950: F, t786: F, t789: F, t159: F, t793: F) -> (F, F, F, F, F, F, F, F) {
    let t7005 = F::cast_from(2.0_f64) * t651 * t7003;
    let t7010 = t30 * t775;
    let t7014 = t212 * t1949;
    let t7015 = t7014 * t780;
    let t7017 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t7015;
    let t7018 = t786 * t1950;
    let t7020 = F::cast_from(0.9757440539382783019e-2_f64) * t7018 * t789;
    let t7021 = t793 * t159;
    (t7005, t7010, t7014, t7015, t7017, t7018, t7020, t7021)
}
