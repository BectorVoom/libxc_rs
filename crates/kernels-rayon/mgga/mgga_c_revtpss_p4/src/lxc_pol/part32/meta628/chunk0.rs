//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2009/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2009(t30410: f64, t686: f64, t72: f64, t93317: f64, t102971: f64, t102974: f64, t102981: f64, t102984: f64, t102988: f64, t102994: f64, t103452: f64, t27353: f64, t28394: f64, t28425: f64, t4487: f64, t62589: f64, t62593: f64, t62628: f64, t95567: f64, t95569: f64, t95576: f64) -> (f64, f64) {
    let t110275 = t30410 * t72 * t686;
    let t110276 = t93317 * t110275;
    let t110281 = t102971 - t102974 + t95567 + t95569 - 0.17347256376410398924e1_f64 * t27353 * t28425 * t62628 - 0.96373646535613327357e-2_f64 * t95576 - 0.68540937416128198419e-2_f64 * t102981 + t102984 - t102988 + 0.26020884564615598386e1_f64 * t27353 * t103452 * t62589 - 0.26020884564615598386e1_f64 * t27353 * t28425 * t62593 - 0.77108554593144223219e-1_f64 * t110276 + 0.26341796731742046394e1_f64 * t28394 * t4487 + 0.19274729307122665472e-1_f64 * t102994;
    (t110275, t110281)
}
