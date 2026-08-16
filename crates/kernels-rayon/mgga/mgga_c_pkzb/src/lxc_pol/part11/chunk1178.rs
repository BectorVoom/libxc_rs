//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1178/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1178(t24604: f64, t24606: f64, t16593: f64, t16595: f64, t16592: f64, t28954: f64, t28955: f64, t28956: f64, t28957: f64, t28958: f64, t28959: f64, t16600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28960 = 0.32530743900905219526e-1_f64 * t24604;
    let t28961 = 0.35089341735807877242e1_f64 * t24606;
    let t28962 = 0.35089341735807877242e1_f64 * t16593;
    let t28963 = 0.21687162600603479684e-1_f64 * t16595;
    let t28964 = t28954 - t28955 - t28956 - t28957 - t28958 + t28959 + t28960 + t28961 - t16592 - t28962 - t28963;
    let t28966 = 0.32530743900905219526e-1_f64 * t16600;
    (t28960, t28961, t28962, t28963, t28964, t28966)
}
