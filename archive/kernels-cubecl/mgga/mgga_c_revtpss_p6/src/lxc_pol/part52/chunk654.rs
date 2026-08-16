//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 654/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk654<F: Float>(t1949: F, t212: F, t780: F, t689: F, t1950: F, t786: F, t789: F, t159: F, t793: F, t218: F, t816: F, t1941: F, t228: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7014 = t212 * t1949;
    let t7015 = t7014 * t780;
    let t7017 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t7015;
    let t7018 = t786 * t1950;
    let t7020 = F::cast_from(0.9757440539382783019e-2_f64) * t7018 * t789;
    let t7021 = t793 * t159;
    let t7023 = t7021 * t218 * t816;
    let t7024 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t7023;
    let t7025 = t1941 * t228;
    (t7014, t7015, t7017, t7018, t7020, t7021, t7023, t7024, t7025)
}
