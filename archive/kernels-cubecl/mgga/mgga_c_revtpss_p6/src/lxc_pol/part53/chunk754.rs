//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 754/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk754<F: Float>(t2014: F, t7901: F, t1873: F, t7252: F, t1885: F, t7264: F, t1889: F, t7271: F, t7251: F, t7258: F, t7261: F, t7268: F) -> (F, F) {
    let t7903 = F::cast_from(3.0_f64) * t2014 * t7901;
    let t7904 = t7252 * t1873;
    let t7906 = t7264 * t1885;
    let t7908 = t7271 * t1889;
    let t7910 = -t7251 - t7904 / F::cast_from(48.0_f64) - t7258 + t7261 - F::cast_from(0.42874018118069736972e-3_f64) * t7906 - t7268 - F::cast_from(0.17149607247227894789e-2_f64) * t7908;
    (t7903, t7910)
}
