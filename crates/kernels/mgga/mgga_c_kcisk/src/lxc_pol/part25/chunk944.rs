//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 944/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk944<F: Float>(t16687: F, t10375: F, t2473: F, t1799: F, t4797: F, t6697: F, t1873: F, t1869: F, t4798: F, t6974: F, t10515: F, t10517: F, t11245: F, t16643: F, t16646: F, t16651: F, t16656: F, t16658: F, t16663: F, t16670: F, t16673: F, t16677: F, t16682: F, t16685: F, t4830: F, t7284: F) -> (F, F, F, F, F) {
    let t16688 = 0.66327777777777777776e-2 * t16687;
    let t16689 = t10375 * t2473;
    let t16690 = t1799 * t16689;
    let t16692 = t6697 * t4797;
    let t16693 = t1873 * t16692;
    let t16694 = t1869 * t16693;
    let t16696 = t6974 * t4798;
    let t16697 = t1869 * t16696;
    let t16699 = -0.22109259259259259258e-2 * t16643 + 0.99491666666666666664e-2 * t16646 + 0.99491666666666666664e-2 * t16651 + 0.49745833333333333332e-2 * t16656 - 0.3684876543209876543e-3 * t16658 - 0.33163888888888888888e-2 * t16663 + 0.386e0 * t4830 * t7284 + 0.148996e0 * t11245 * t7284 - 0.33163888888888888888e-2 * t16670 + t16673 + 0.22109259259259259258e-2 * t16677 + 0.11054629629629629629e-2 * t10515 - 0.11054629629629629629e-2 * t10517 + 0.13265555555555555555e-1 * t16682 - 0.88437037037037037034e-2 * t16685 + t16688 + 0.16581944444444444444e-2 * t16690 + 0.66327777777777777776e-2 * t16694 - 0.24872916666666666666e-2 * t16697;
    (t16690, t16692, t16694, t16697, t16699)
}
