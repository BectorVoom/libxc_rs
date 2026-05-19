//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 815/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk815<F: Float>(t4130: F, t5748: F, t1464: F, t11780: F, t11799: F, t11811: F, t11815: F, t15800: F, t15804: F, t15810: F, t15813: F, t15817: F, t15821: F, t15824: F, t15826: F, t15830: F, t15832: F, t15836: F, t15840: F, t15844: F) -> (F, F) {
    let t15846 = t5748 * t4130;
    let t15847 = t1464 * t15846;
    let t15849 = F::cast_from(0.55273148148148148147e-3_f64) * t15800 - F::cast_from(0.33163888888888888888e-2_f64) * t15804 + F::cast_from(0.22109259259259259258e-2_f64) * t11780 - F::cast_from(0.33163888888888888888e-2_f64) * t11799 + F::cast_from(0.33163888888888888888e-2_f64) * t15810 - F::cast_from(0.24872916666666666666e-2_f64) * t15813 + F::cast_from(0.66327777777777777776e-2_f64) * t15817 - F::cast_from(0.13265555555555555555e-1_f64) * t15821 + F::cast_from(0.13265555555555555555e-1_f64) * t15824 - F::cast_from(0.3684876543209876543e-3_f64) * t15826 + F::cast_from(0.11054629629629629629e-2_f64) * t15830 - F::cast_from(0.33163888888888888888e-2_f64) * t15832 + F::cast_from(0.18424382716049382715e-2_f64) * t15836 - F::cast_from(0.16581944444444444444e-2_f64) * t15840 + F::cast_from(0.11054629629629629629e-2_f64) * t11811 + F::cast_from(0.18424382716049382715e-2_f64) * t11815 + F::cast_from(0.22109259259259259258e-2_f64) * t15844 - F::cast_from(0.33163888888888888888e-2_f64) * t15847;
    (t15847, t15849)
}
