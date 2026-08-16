//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1392/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1392(t1889: f64, t4457: f64, t12605: f64, t12827: f64, t12830: f64, t12834: f64, t12840: f64, t12842: f64, t12846: f64, t12849: f64, t18069: f64, t18071: f64, t18080: f64, t18083: f64, t4439: f64) -> f64 {
    let t18086 = t1889 * t4457;
    let t18087 = t12605 * t18086;
    let t18090 = -t12846 / 864.0_f64 + t18069 / 324.0_f64 + t4439 * t18071 / 96.0_f64 + t12840 - t12827 / 1296.0_f64 + t12830 / 1728.0_f64 + t12834 / 1296.0_f64 - t12849 / 864.0_f64 + t12842 / 432.0_f64 + t4439 * t18080 / 72.0_f64 - t4439 * t18083 / 72.0_f64 + t4439 * t18087 / 288.0_f64;
    t18090
}
