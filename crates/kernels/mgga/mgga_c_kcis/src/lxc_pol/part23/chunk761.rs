//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 761/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk761<F: Float>(t2046: F, t3797: F, t4170: F, t5661: F, t2038: F, t3805: F, t4162: F, t4160: F, t4142: F, t5773: F, t4130: F, t5748: F, t1464: F, t11780: F, t11799: F, t11811: F, t11815: F, t15800: F, t15804: F, t15810: F, t15813: F, t15817: F, t15821: F, t15824: F, t15826: F, t15830: F, t15832: F) -> (F, F, F, F, F, F, F) {
    let t15834 = t2046 * t3797;
    let t15835 = t4170 * t15834;
    let t15836 = t5661 * t15835;
    let t15838 = t2038 * t3805;
    let t15839 = t4162 * t15838;
    let t15840 = t4160 * t15839;
    let t15844 = t4142 * t5773;
    let t15846 = t5748 * t4130;
    let t15847 = t1464 * t15846;
    let t15849 = 0.55273148148148148147e-3 * t15800 - 0.33163888888888888888e-2 * t15804 + 0.22109259259259259258e-2 * t11780 - 0.33163888888888888888e-2 * t11799 + 0.33163888888888888888e-2 * t15810 - 0.24872916666666666666e-2 * t15813 + 0.66327777777777777776e-2 * t15817 - 0.13265555555555555555e-1 * t15821 + 0.13265555555555555555e-1 * t15824 - 0.3684876543209876543e-3 * t15826 + 0.11054629629629629629e-2 * t15830 - 0.33163888888888888888e-2 * t15832 + 0.18424382716049382715e-2 * t15836 - 0.16581944444444444444e-2 * t15840 + 0.11054629629629629629e-2 * t11811 + 0.18424382716049382715e-2 * t11815 + 0.22109259259259259258e-2 * t15844 - 0.33163888888888888888e-2 * t15847;
    (t15834, t15836, t15838, t15840, t15844, t15847, t15849)
}
