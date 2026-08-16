//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1262/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1262(t54128: f64, t54135: f64, t54152: f64, t54166: f64, t54198: f64, t54236: f64, t54238: f64, t54257: f64, t54259: f64, t54267: f64, t54271: f64, t54283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55487 = 7.0_f64 / 288.0_f64 * t54128;
    let t55491 = 7.0_f64 / 72.0_f64 * t54135;
    let t55500 = 7.0_f64 / 72.0_f64 * t54152;
    let t55508 = 7.0_f64 / 72.0_f64 * t54166;
    let t55524 = 7.0_f64 / 288.0_f64 * t54198;
    let t55547 = 7.0_f64 / 72.0_f64 * t54236;
    let t55548 = 7.0_f64 / 144.0_f64 * t54238;
    let t55556 = 7.0_f64 / 72.0_f64 * t54257;
    let t55557 = 7.0_f64 / 36.0_f64 * t54259;
    let t55562 = 7.0_f64 / 36.0_f64 * t54267;
    let t55564 = 7.0_f64 / 72.0_f64 * t54271;
    let t55569 = 7.0_f64 / 288.0_f64 * t54283;
    (t55487, t55491, t55500, t55508, t55524, t55547, t55548, t55556, t55557, t55562, t55564, t55569)
}
