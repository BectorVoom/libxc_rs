//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1227/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1227(t2038: f64, t3805: f64, t4162: f64, t4160: f64, t4142: f64, t5773: f64, t4130: f64, t5748: f64, t1464: f64, t11780: f64, t11799: f64, t11811: f64, t11815: f64, t15800: f64, t15804: f64, t15810: f64, t15813: f64, t15817: f64, t15821: f64, t15824: f64, t15826: f64, t15830: f64, t15832: f64, t15836: f64) -> (f64, f64, f64, f64) {
    let t15838 = t2038 * t3805;
    let t15839 = t4162 * t15838;
    let t15840 = t4160 * t15839;
    let t15844 = t4142 * t5773;
    let t15846 = t5748 * t4130;
    let t15847 = t1464 * t15846;
    let t15849 = 0.55273148148148148147e-3_f64 * t15800 - 0.33163888888888888888e-2_f64 * t15804 + 0.22109259259259259258e-2_f64 * t11780 - 0.33163888888888888888e-2_f64 * t11799 + 0.33163888888888888888e-2_f64 * t15810 - 0.24872916666666666666e-2_f64 * t15813 + 0.66327777777777777776e-2_f64 * t15817 - 0.13265555555555555555e-1_f64 * t15821 + 0.13265555555555555555e-1_f64 * t15824 - 0.3684876543209876543e-3_f64 * t15826 + 0.11054629629629629629e-2_f64 * t15830 - 0.33163888888888888888e-2_f64 * t15832 + 0.18424382716049382715e-2_f64 * t15836 - 0.16581944444444444444e-2_f64 * t15840 + 0.11054629629629629629e-2_f64 * t11811 + 0.18424382716049382715e-2_f64 * t11815 + 0.22109259259259259258e-2_f64 * t15844 - 0.33163888888888888888e-2_f64 * t15847;
    (t15840, t15844, t15847, t15849)
}
