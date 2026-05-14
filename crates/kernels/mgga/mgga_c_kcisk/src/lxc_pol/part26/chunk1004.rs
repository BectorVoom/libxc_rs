//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1004/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1004<F: Float>(t26941: F, t3482: F, t19926: F, t19948: F, t19966: F, t26892: F, t26896: F, t26899: F, t26903: F, t26907: F, t26912: F, t26914: F, t26917: F, t26919: F, t26922: F, t26925: F, t26930: F, t26934: F, t26936: F, t3491: F, t8064: F) -> (F, F) {
    let t26942 = t3482 * t26941;
    let t26944 = -0.7369753086419753086e-3 * t19926 + 0.55273148148148148147e-3 * t26892 + 0.73697530864197530862e-3 * t26896 + 0.33163888888888888888e-2 * t26899 - 0.36848765432098765431e-3 * t26903 - 0.13265555555555555555e-1 * t26907 - 0.7369753086419753086e-3 * t19948 + 0.3684876543209876543e-2 * t26912 - 0.36848765432098765431e-3 * t26914 + 0.49745833333333333332e-2 * t26917 + 0.22109259259259259259e-2 * t26919 - 0.88437037037037037035e-2 * t26922 - 0.33163888888888888888e-2 * t26925 - 0.1492375e-1 * t26930 - 0.88437037037037037035e-2 * t19966 - 0.66327777777777777776e-2 * t26934 + 0.16581944444444444444e-2 * t26936 + 0.193e0 * t3491 * t8064 - 0.22109259259259259258e-2 * t26942;
    (t26942, t26944)
}
