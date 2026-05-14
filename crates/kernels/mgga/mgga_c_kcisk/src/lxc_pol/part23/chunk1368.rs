//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1368/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1368<F: Float>(t1339: F, t33604: F, t3487: F, t33352: F, t3748: F, t33349: F, t110459: F, t110492: F, t113871: F, t114107: F, t114111: F, t114113: F, t114117: F, t114121: F, t114125: F, t114131: F, t32022: F, t32087: F, t32096: F, t33346: F, t33588: F, t9429: F, t9796: F, t9805: F) -> (F, F, F, F) {
    let t114134 = t1339 * t33604 * t3487;
    let t114136 = t3748 * t33352;
    let t114138 = t3748 * t33349;
    let t114139 = 0.14739506172839506172e-2 * t114138;
    let t114142 = 0.20833333333333333334e-1 * t32096 * t33346 + 0.8041666666666666667e-2 * t114107 * t9429 + t114111 - 0.11054629629629629629e-2 * t114113 - 0.55273148148148148147e-3 * t114117 - 0.55555555555555555558e-1 * t32022 * t33588 - 0.21444444444444444446e-1 * t114121 * t9429 + t114125 - 0.21444444444444444446e-1 * t110459 * t9796 - 0.69444444444444444446e-2 * t32087 * t113871 + 0.27636574074074074073e-2 * t114131 + 0.22109259259259259258e-2 * t114134 - 0.22109259259259259258e-2 * t114136 + t114139 - 0.34722222222222222223e-2 * t110492 * t9805;
    (t114134, t114136, t114138, t114142)
}
