//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3876/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3876(t22126: f64, t2689: f64, t22130: f64, t13867: f64, t47248: f64, t48712: f64, t48855: f64, t5704: f64, t74461: f64, t74469: f64, t74471: f64, t74475: f64, t74479: f64, t74481: f64, t74485: f64, t74489: f64) -> f64 {
    let t74491 = t2689 * t22126;
    let t74493 = t2689 * t22130;
    let t74496 = 0.2032800112371413129e-3_f64 * t74461 - 0.10289764348336736873e0_f64 * t48712 * t47248 * t5704 * t13867 + 0.50820002809285328225e-4_f64 * t74469 - 0.80031500487063509015e-2_f64 * t74471 - 0.57165357490759649296e-4_f64 * t74475 + 0.50820002809285328225e-3_f64 * t74479 - 0.40015750243531754508e-1_f64 * t74481 + 0.36143185997963725434e-4_f64 * t74485 - 0.50820002809285328226e-3_f64 * t74489 - 0.30488190661738479625e-3_f64 * t74491 + 0.15244095330869239812e-2_f64 * t74493 + 0.40015750243531754508e-1_f64 * t48855;
    t74496
}
