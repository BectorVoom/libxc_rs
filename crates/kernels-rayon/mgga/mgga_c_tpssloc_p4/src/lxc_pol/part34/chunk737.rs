//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 737/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk737(t236: f64, t9971: f64, t240: f64, t812: f64, t232: f64, t2632: f64, t597: f64, t61: f64, t241: f64, t244: f64, t248: f64, t238: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9972 = t9971 * t236;
    let t9973 = t9972 * t240;
    let t9974 = t812 * t9973;
    let t9975 = t2632 * t232;
    let t10021 = 1.0_f64 / t61 / t597;
    let t10022 = t10021 * t241;
    let t10024 = t10022 * t244 * t248;
    let t10026 = 595.0_f64 / 10368.0_f64 * t238 * t10024;
    (t9972, t9974, t9975, t10022, t10024, t10026)
}
