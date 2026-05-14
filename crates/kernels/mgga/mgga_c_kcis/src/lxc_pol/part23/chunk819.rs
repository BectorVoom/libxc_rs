//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 819/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk819<F: Float>(t1468: F, t16649: F, t1464: F, t2011: F, t3954: F, t1495: F, t3722: F, t4135: F, t1395: F, t3728: F, t5877: F, t1489: F, t5627: F, t1396: F, t4123: F, t11914: F, t1364: F, t15978: F, t15987: F, t15989: F, t16612: F, t16615: F, t16620: F, t16625: F, t16628: F, t16629: F, t16632: F, t16636: F, t16640: F, t16644: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16650 = t1468 * t16649;
    let t16651 = t1464 * t16650;
    let t16653 = t2011 * t3954;
    let t16654 = t1495 * t16653;
    let t16655 = t1468 * t16654;
    let t16656 = t1464 * t16655;
    let t16658 = t2011 * t3722;
    let t16659 = t4135 * t16658;
    let t16660 = t1395 * t16659;
    let t16661 = t1464 * t16660;
    let t16663 = t3728 * t5877;
    let t16665 = t5627 * t1489;
    let t16666 = t1396 * t16665;
    let t16667 = t4123 * t16666;
    let t16668 = t1464 * t16667;
    let t16670 = -t15987 - t15989 - 0.24872916666666666666e-2 * t16612 - 0.55273148148148148147e-3 * t16615 + 0.14739506172839506172e-2 * t16620 + 0.49745833333333333332e-2 * t16625 + t16628 - 0.5895802469135802469e-2 * t16629 - t16632 - 0.73697530864197530861e-3 * t16636 - 0.22109259259259259258e-2 * t16640 - 0.22109259259259259258e-2 * t16644 - 0.22109259259259259258e-2 * t11914 + 0.66725e-1 * t1364 * t15978 + 0.88437037037037037034e-2 * t16651 - 0.16581944444444444444e-2 * t16656 - 0.55273148148148148147e-3 * t16661 - 0.73697530864197530861e-3 * t16663 + 0.99491666666666666664e-2 * t16668;
    (t16651, t16653, t16656, t16658, t16661, t16663, t16665, t16668, t16670)
}
