//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1366/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1366<F: Float>(t32203: F, t3759: F, t5635: F, t110477: F, t113765: F, t113997: F, t114072: F, t114075: F, t114082: F, t114092: F, t114095: F, t32072: F, t32189: F, t32216: F, t33373: F, t33384: F, t33389: F, t33400: F, t33460: F, t9426: F, t9429: F, t9449: F) -> (F, F) {
    let t114098 = t3759 * t32203 * t5635;
    let t114103 = -0.20833333333333333334e-1 * t33384 * t32072 - 0.8041666666666666667e-2 * t33460 * t32072 - 0.16581944444444444444e-1 * t114072 - 0.69444444444444444446e-2 * t114075 * t9449 + 0.21444444444444444446e-1 * t32189 * t33400 + 0.8041666666666666667e-2 * t9426 * t114082 + 0.64333333333333333336e-1 * t32189 * t33389 + 0.69444444444444444446e-2 * t33373 * t32216 - 0.120625e-1 * t9426 * t113765 - 0.58958024691358024689e-2 * t114092 + 0.16203703703703703704e-1 * t114095 + 0.55273148148148148146e-2 * t114098 - 0.40208333333333333334e-2 * t110477 - 0.55555555555555555558e-1 * t113997 * t9429;
    (t114098, t114103)
}
