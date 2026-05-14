//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1011/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1011<F: Float>(t33982: F, t33984: F, t33986: F, t33994: F, t33996: F, t34009: F, t34013: F, t33990: F, t33998: F, t34000: F, t34003: F, t34005: F, t34011: F, t34015: F, t34017: F, t34019: F, t34021: F, t34023: F) -> (F,) {
    let t36888 = 0.12862205435420921092e-1 * t33982;
    let t36889 = 0.37737710747524982482e-2 * t33984;
    let t36890 = 0.12579236915841660828e-2 * t33986;
    let t36892 = 0.14291339372689912324e-2 * t33994;
    let t36893 = 0.85748036236139473944e-3 * t33996;
    let t36898 = 0.42874018118069736972e-3 * t34009;
    let t36900 = 0.12862205435420921092e-1 * t34013;
    let t36906 = -t36888 + t36889 + t36890 - 0.15095084299009992993e-1 * t33990 + t36892 + t36893 + 0.68598428988911579156e-2 * t33998 - 0.10289764348336736873e-1 * t34000 + 0.85748036236139473944e-3 * t34003 - 0.20579528696673473747e-1 * t34005 + t36898 - 0.83861579438944405518e-2 * t34011 + t36900 + 0.34299214494455789578e-2 * t34015 + 0.17149607247227894789e-2 * t34017 - 0.34299214494455789578e-2 * t34019 - 0.85748036236139473944e-3 * t34021 + 0.56606566121287473724e-1 * t34023;
    (t36906,)
}
