//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1161/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1161(t33982: f64, t33984: f64, t33986: f64, t33994: f64, t33996: f64, t34009: f64, t34013: f64, t33990: f64, t33998: f64, t34000: f64, t34003: f64, t34005: f64, t34011: f64, t34015: f64, t34017: f64, t34019: f64, t34021: f64, t34023: f64) -> f64 {
    let t36888 = 0.12862205435420921092e-1_f64 * t33982;
    let t36889 = 0.37737710747524982482e-2_f64 * t33984;
    let t36890 = 0.12579236915841660828e-2_f64 * t33986;
    let t36892 = 0.14291339372689912324e-2_f64 * t33994;
    let t36893 = 0.85748036236139473944e-3_f64 * t33996;
    let t36898 = 0.42874018118069736972e-3_f64 * t34009;
    let t36900 = 0.12862205435420921092e-1_f64 * t34013;
    let t36906 = -t36888 + t36889 + t36890 - 0.15095084299009992993e-1_f64 * t33990 + t36892 + t36893 + 0.68598428988911579156e-2_f64 * t33998 - 0.10289764348336736873e-1_f64 * t34000 + 0.85748036236139473944e-3_f64 * t34003 - 0.20579528696673473747e-1_f64 * t34005 + t36898 - 0.83861579438944405518e-2_f64 * t34011 + t36900 + 0.34299214494455789578e-2_f64 * t34015 + 0.17149607247227894789e-2_f64 * t34017 - 0.34299214494455789578e-2_f64 * t34019 - 0.85748036236139473944e-3_f64 * t34021 + 0.56606566121287473724e-1_f64 * t34023;
    t36906
}
