//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 903/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk903(t2873: f64, t978: f64, t10: f64, t10051: f64, t10054: f64, t10065: f64, t10069: f64, t10072: f64, t10075: f64, t10078: f64, t10081: f64, t496: f64, t5749: f64, t5751: f64, t5753: f64, t5755: f64, t5759: f64, t5764: f64, t5776: f64, t8126: f64, t8137: f64, t8139: f64, t8142: f64) -> f64 {
    let t10085 = t978 * t2873;
    let t10089 = -t5749 - t5751 + t5753 - t5755 - t5759 - t10051 + t10054 - t496 * t10065 / 2.0_f64 - t10069 / 2.0_f64 + t10072 / 6.0_f64 - 0.293808e1_f64 * t10075 + 0.73452e0_f64 * t10078 - 0.48968000000000000001e0_f64 * t5764 - 6.0_f64 * t496 * t10 * t10081 + 3.0_f64 * t496 * t10 * t10085 - t5776 - t8126 - t8137 - t8139 + t8142;
    t10089
}
