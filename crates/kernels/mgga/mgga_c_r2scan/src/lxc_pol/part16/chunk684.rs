//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 684/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk684<F: Float>(t4741: F, t5309: F, t5312: F, t5315: F, t171: F, t584: F, t61: F, t718: F, t226: F, t5456: F, t160: F, t35: F, t164: F, t1774: F, t604: F, t1780: F, t601: F) -> (F, F, F, F, F, F) {
    let t5860 = 0.32547666666666666667e-1 * t4741;
    let t5861 = -0.14816666666666666667e-1 * t5309 + 0.9877777777777777778e-2 * t5312 - 0.46096296296296296297e-1 * t5315 - t5860;
    let t5864 = 0.571528e-1 * t584 * t171 * t5861;
    let t5865 = t61 * t718;
    let t5866 = t226 * t5456;
    let t5868 = 0.10526802520742363173e2 * t5865 * t5866;
    let t5869 = t160 * t35;
    let t5871 = 1320.0 * t5869 * t164;
    let t5872 = t1774 * t604;
    let t5874 = t601 * t1780;
    (t5860, t5864, t5868, t5871, t5872, t5874)
}
