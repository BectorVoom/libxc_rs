//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3261/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3261(t18352: f64, t2710: f64, t2713: f64, t10722: f64, t6030: f64, t18419: f64, t9775: f64, t14791: f64, t14802: f64, t40679: f64, t40681: f64, t40691: f64, t40707: f64, t40711: f64, t40722: f64, t4362: f64, t50703: f64, t50706: f64, t6022: f64) -> f64 {
    let t61888 = t2710 * t2713 * t18352;
    let t61890 = t10722 * t6030;
    let t61892 = t9775 * t18419;
    let t61899 = 0.65057734796334705782e-3_f64 * t50703 - 0.2032800112371413129e-3_f64 * t50706 + 0.15244095330869239812e-3_f64 * t40679 - 0.27104001498285508386e-2_f64 * t40681 + 0.22589491248727328396e-6_f64 * t40691 - 0.22675591804667994221e-1_f64 * t40707 - 0.10276933901433255263e-1_f64 * t40711 + 0.90357964994909313586e-4_f64 * t61888 - 0.22675591804667994221e-1_f64 * t61890 - 0.76220476654346199061e-4_f64 * t61892 - 0.3659040202268543632e-3_f64 * t40722 - 0.10289764348336736874e-1_f64 * t4362 * t14791 * t6022 * t14802;
    t61899
}
