//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 937/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk937<F: Float>(t5074: F, t6720: F, t10379: F, t10410: F, t10417: F, t15919: F, t15924: F, t15933: F, t15939: F, t15942: F, t15945: F, t15949: F, t15951: F, t15953: F, t15955: F, t15958: F, t16572: F, t16578: F, t16583: F, t16586: F) -> (F, F) {
    let t16588 = t5074 * t6720;
    let t16590 = -0.11054629629629629629e-1 * t15919 + 0.27636574074074074073e-2 * t15924 + 0.73697530864197530861e-2 * t15933 + 0.22109259259259259258e-2 * t10379 + 0.14739506172839506172e-2 * t15939 - 0.49745833333333333332e-2 * t15942 - 0.55273148148148148147e-3 * t15945 - 0.33163888888888888888e-2 * t15949 + 0.55273148148148148147e-3 * t15951 - 0.3684876543209876543e-3 * t15953 + 0.14739506172839506172e-2 * t15955 - 0.66327777777777777776e-2 * t15958 + 0.24872916666666666666e-2 * t16572 + 0.14739506172839506172e-2 * t10410 - 0.22109259259259259258e-2 * t10417 - 0.22109259259259259258e-2 * t16578 - 0.73697530864197530861e-3 * t16583 - 0.33163888888888888888e-2 * t16586 + 0.22109259259259259258e-2 * t16588;
    (t16588, t16590)
}
