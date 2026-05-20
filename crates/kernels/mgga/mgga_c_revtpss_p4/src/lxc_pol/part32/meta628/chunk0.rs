//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2009/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2009<F: Float>(t30410: F, t686: F, t72: F, t93317: F, t102971: F, t102974: F, t102981: F, t102984: F, t102988: F, t102994: F, t103452: F, t27353: F, t28394: F, t28425: F, t4487: F, t62589: F, t62593: F, t62628: F, t95567: F, t95569: F, t95576: F) -> (F, F) {
    let t110275 = t30410 * t72 * t686;
    let t110276 = t93317 * t110275;
    let t110281 = t102971 - t102974 + t95567 + t95569 - F::cast_from(0.17347256376410398924e1_f64) * t27353 * t28425 * t62628 - F::cast_from(0.96373646535613327357e-2_f64) * t95576 - F::cast_from(0.68540937416128198419e-2_f64) * t102981 + t102984 - t102988 + F::cast_from(0.26020884564615598386e1_f64) * t27353 * t103452 * t62589 - F::cast_from(0.26020884564615598386e1_f64) * t27353 * t28425 * t62593 - F::cast_from(0.77108554593144223219e-1_f64) * t110276 + F::cast_from(0.26341796731742046394e1_f64) * t28394 * t4487 + F::cast_from(0.19274729307122665472e-1_f64) * t102994;
    (t110275, t110281)
}
