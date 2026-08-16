//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1972/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1972(t30266: f64, t689: f64, t25904: f64, t109412: f64, t25878: f64, t109403: f64, t94669: f64, t102143: f64, t102164: f64, t102167: f64, t1398: f64, t27837: f64, t28830: f64, t30247: f64, t543: f64, t5658: f64, t7295: f64, t7301: f64, t8085: f64, t96210: f64, t96211: f64, t96218: f64, t96222: f64, t96230: f64) -> (f64, f64) {
    let t109425 = t30266 * t689;
    let t109426 = t25904 * t109425;
    let t109434 = t25878 * t109412;
    let t109437 = t94669 * t109403;
    let t109446 = -0.72280234901709995518e-2_f64 * t109426 + t102143 - t96210 - 0.96373646535613327357e-2_f64 * t96211 + 0.8673628188205199462e0_f64 * t7295 * t7301 * t8085 * t5658 * t543 - t96218 + 0.51405703062096148813e-1_f64 * t109434 + 0.22849835011101738147e-2_f64 * t96222 + t102164 - 0.77108554593144223219e-1_f64 * t109437 + t96230 + t102167 + 0.8673628188205199462e0_f64 * t27837 * t28830 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t30247 * t1398 * t543;
    (t109425, t109446)
}
