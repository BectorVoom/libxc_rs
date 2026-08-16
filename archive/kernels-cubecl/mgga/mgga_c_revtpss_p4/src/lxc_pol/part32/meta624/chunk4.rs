//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1972/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1972<F: Float>(t30266: F, t689: F, t25904: F, t109412: F, t25878: F, t109403: F, t94669: F, t102143: F, t102164: F, t102167: F, t1398: F, t27837: F, t28830: F, t30247: F, t543: F, t5658: F, t7295: F, t7301: F, t8085: F, t96210: F, t96211: F, t96218: F, t96222: F, t96230: F) -> (F, F) {
    let t109425 = t30266 * t689;
    let t109426 = t25904 * t109425;
    let t109434 = t25878 * t109412;
    let t109437 = t94669 * t109403;
    let t109446 = -F::cast_from(0.72280234901709995518e-2_f64) * t109426 + t102143 - t96210 - F::cast_from(0.96373646535613327357e-2_f64) * t96211 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t8085 * t5658 * t543 - t96218 + F::cast_from(0.51405703062096148813e-1_f64) * t109434 + F::cast_from(0.22849835011101738147e-2_f64) * t96222 + t102164 - F::cast_from(0.77108554593144223219e-1_f64) * t109437 + t96230 + t102167 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t28830 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t30247 * t1398 * t543;
    (t109425, t109446)
}
