//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2012/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2012<F: Float>(t30380: F, t686: F, t72: F, t7058: F, t28314: F, t99466: F, t7064: F, t103086: F, t103088: F, t103103: F, t103114: F, t103119: F, t103122: F, t103130: F, t103136: F, t25383: F, t27199: F, t28310: F, t30411: F, t95740: F, t95747: F) -> F {
    let t110339 = t30380 * t72 * t686;
    let t110340 = t7058 * t110339;
    let t110344 = t99466 * t28314;
    let t110346 = t7064 * t110339;
    let t110348 = F::cast_from(0.17347256376410398924e1_f64) * t27199 * t28310 - F::cast_from(0.24093411633903331839e-3_f64) * t95740 + F::cast_from(0.22849835011101738147e-2_f64) * t95747 - t103086 + t103088 - F::cast_from(0.26020884564615598386e1_f64) * t25383 * t30411 - t103103 - F::cast_from(0.19274729307122665472e-1_f64) * t103114 + t103119 + F::cast_from(0.72280234901709995518e-2_f64) * t110340 + F::cast_from(0.45699670022203476294e-2_f64) * t103122 + F::cast_from(0.4818682326780666368e-3_f64) * t103130 - F::cast_from(0.28912093960683998207e-1_f64) * t110344 - t103136 - F::cast_from(0.12851425765524037203e-1_f64) * t110346;
    t110348
}
