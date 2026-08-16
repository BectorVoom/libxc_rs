//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1211/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1211(t35997: f64, t35965: f64, t35971: f64, t35991: f64, t35995: f64, t35999: f64, t37806: f64, t37807: f64, t37808: f64, t37810: f64, t37811: f64, t37813: f64, t37814: f64, t37815: f64, t37816: f64, t37817: f64, t37818: f64, t37819: f64) -> f64 {
    let t37822 = 0.18868855373762491241e-1_f64 * t35997;
    let t37824 = t37806 + t37807 + t37808 + 0.17149607247227894789e-2_f64 * t35965 - t37810 + t37811 - 0.17149607247227894789e-2_f64 * t35971 - t37813 - t37814 + t37815 - t37816 + t37817 + t37818 + t37819 + 0.41930789719472202757e-2_f64 * t35991 - 0.62896184579208304135e-2_f64 * t35995 - t37822 + 0.68598428988911579156e-2_f64 * t35999;
    t37824
}
