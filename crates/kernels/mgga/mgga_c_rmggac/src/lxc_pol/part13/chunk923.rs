//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 923/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk923<F: Float>(t39591: F, t2265: F, t5026: F, t1685: F, t2212: F, t2231: F, t2604: F, t30221: F, t35149: F, t35152: F, t35184: F, t35188: F, t39577: F, t39584: F, t39589: F, t39595: F, t39600: F, t39605: F, t39607: F, t39609: F, t5898: F, t72: F, t8264: F, t884: F, t9383: F) -> (F,) {
    let t43042 = 0.1489760996265424379e-3 * t39591;
    let t43043 = t5026 * t2265;
    let t43059 = -0.23948483403727617128e0 * t884 * t8264 * t5898 - 0.14897609962654243789e-3 * t35149 - 0.85129199786595678799e-5 * t39577 + 0.49658699875514145964e-4 * t35152 + 0.2553875993597870364e-4 * t39584 - 0.71827762319940103988e-4 * t39589 - t43042 - 0.2363e1 * t43043 - 0.11974241701863808564e0 * t2604 * t9383 - 0.5987120850931904282e-1 * t39595 + 2.0 * t72 * t1685 * t2231 + 0.2553875993597870364e-4 * t39600 + 0.85129199786595678799e-5 * t39605 - 0.85129199786595678799e-5 * t39607 + 0.79828278012425390428e-1 * t30221 * t2212 - 0.1440846329149835838e-2 * t39609 - 0.10909864661698136692e0 * t35184 - 0.54549323308490683461e-1 * t35188;
    (t43059,)
}
