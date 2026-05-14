//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1139/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1139<F: Float>(t2718: F, t32022: F, t32087: F, t32153: F, t33400: F, t33428: F, t33493: F, t33496: F, t33499: F, t33501: F, t33510: F, t33513: F, t33515: F, t9426: F, t9796: F, t20: F, t394: F, t6147: F) -> (F, F) {
    let t33518 = -0.27777777777777777779e-1 * t32022 * t9796 + 0.16581944444444444444e-2 * t33493 + 0.16581944444444444444e-2 * t33496 - 0.24872916666666666666e-2 * t33499 + 0.11054629629629629629e-2 * t33501 - 0.40208333333333333335e-2 * t9426 * t33400 + 0.34722222222222222223e-2 * t32087 * t33428 + 0.16581944444444444444e-2 * t32153 - 0.16581944444444444444e-2 * t33510 + 0.66327777777777777776e-2 * t33513 - 0.10416666666666666667e-1 * t33515 * t2718;
    let t33520 = t6147 * t394 * t20;
    (t33518, t33520)
}
