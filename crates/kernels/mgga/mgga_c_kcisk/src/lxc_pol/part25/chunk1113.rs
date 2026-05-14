//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1113/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1113<F: Float>(t1791: F, t695: F, t17300: F, t5015: F, t32942: F, t33005: F, t33019: F, t33023: F, t33029: F, t33031: F, t33035: F, t33042: F, t33046: F, t33050: F, t33052: F, t33056: F, t9649: F, t9664: F, t9667: F) -> (F, F, F, F) {
    let t33059 = t1791 * t695;
    let t33060 = t33059 * t17300;
    let t33061 = t5015 * t33060;
    let t33064 = -0.69444444444444444446e-2 * t32942 * t9667 - 0.33163888888888888888e-2 * t33019 - 0.20833333333333333334e-1 * t9664 * t33023 - 0.33163888888888888888e-2 * t33029 + 0.69444444444444444446e-2 * t33031 * t33035 - 0.20833333333333333334e-1 * t9664 * t33005 + 0.16581944444444444444e-2 * t33042 + 0.27636574074074074073e-2 * t33046 - 0.33163888888888888888e-2 * t33050 + 0.26805555555555555556e-2 * t33052 - 0.120625e-1 * t9649 * t33005 + 0.26805555555555555556e-2 * t33056 * t33035 + 0.69444444444444444446e-2 * t33031 * t33061;
    (t33059, t33060, t33061, t33064)
}
