//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1226/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1226<F: Float>(t18210: F, t29323: F, t7898: F, t29407: F, t7904: F, t102621: F, t102626: F, t102629: F, t102632: F, t102636: F, t103258: F, t103502: F, t28535: F, t8151: F, t94626: F, t98744: F) -> (F, F) {
    let t103662 = t18210 * t29323;
    let t103663 = t7898 * t103662;
    let t103665 = t29407 * t7904;
    let t103669 = -0.22109259259259259258e-2 * t102621 - 0.22109259259259259259e-2 * t102626 + 0.66327777777777777776e-2 * t102629 - 0.46336805555555555556e-3 * t94626 * t103258 - 0.92673611111111111112e-3 * t94626 * t103502 - 0.37069444444444444444e-2 * t8151 * t28535 - 0.92754700520833333333e-4 * t103663 + t98744 + 0.12356481481481481481e-2 * t103665 + 0.24872916666666666666e-2 * t102632 - 0.88437037037037037033e-2 * t102636;
    (t103662, t103669)
}
