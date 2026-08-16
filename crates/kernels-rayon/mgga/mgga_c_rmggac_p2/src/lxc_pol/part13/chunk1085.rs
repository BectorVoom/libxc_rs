//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1085/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1085(t41363: f64, t41365: f64, t41371: f64, t41373: f64, t41377: f64, t41379: f64, t41381: f64, t36175: f64, t36184: f64, t36188: f64, t36190: f64, t36192: f64, t36194: f64, t37558: f64, t37560: f64, t41375: f64) -> f64 {
    let t43622 = 0.66671395154821946449e-1_f64 * t41363;
    let t43623 = 0.17740875559651324989e-2_f64 * t41365;
    let t43628 = 0.10643770401656718724e0_f64 * t41371;
    let t43629 = 0.10643770401656718724e0_f64 * t41373;
    let t43631 = 0.36366215538993788972e-1_f64 * t41377;
    let t43632 = 0.48488287385325051964e-1_f64 * t41379;
    let t43633 = 0.11289648083414479539e-2_f64 * t41381;
    let t43634 = -0.21241846568096930143e-2_f64 * t36175 + 0.70806155226989767144e-2_f64 * t36184 + t43622 + t43623 - 0.12981128458281457309e-1_f64 * t36188 + 0.15577354149937748771e-1_f64 * t36190 + 0.148692925976678511e-1_f64 * t36192 + 0.17701538806747441786e-2_f64 * t36194 + t37558 + t43628 + t43629 - t37560 - 0.21168090156402149135e-3_f64 * t41375 - t43631 + t43632 + t43633;
    t43634
}
