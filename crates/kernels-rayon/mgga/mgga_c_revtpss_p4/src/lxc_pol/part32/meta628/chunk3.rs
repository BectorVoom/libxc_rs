//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2012/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2012(t30380: f64, t686: f64, t72: f64, t7058: f64, t28314: f64, t99466: f64, t7064: f64, t103086: f64, t103088: f64, t103103: f64, t103114: f64, t103119: f64, t103122: f64, t103130: f64, t103136: f64, t25383: f64, t27199: f64, t28310: f64, t30411: f64, t95740: f64, t95747: f64) -> f64 {
    let t110339 = t30380 * t72 * t686;
    let t110340 = t7058 * t110339;
    let t110344 = t99466 * t28314;
    let t110346 = t7064 * t110339;
    let t110348 = 0.17347256376410398924e1_f64 * t27199 * t28310 - 0.24093411633903331839e-3_f64 * t95740 + 0.22849835011101738147e-2_f64 * t95747 - t103086 + t103088 - 0.26020884564615598386e1_f64 * t25383 * t30411 - t103103 - 0.19274729307122665472e-1_f64 * t103114 + t103119 + 0.72280234901709995518e-2_f64 * t110340 + 0.45699670022203476294e-2_f64 * t103122 + 0.4818682326780666368e-3_f64 * t103130 - 0.28912093960683998207e-1_f64 * t110344 - t103136 - 0.12851425765524037203e-1_f64 * t110346;
    t110348
}
