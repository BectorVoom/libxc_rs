//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 807/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk807<F: Float>(t1468: F, t20926: F, t1464: F, t1364: F, t15800: F, t15826: F, t16744: F, t1944: F, t20875: F, t20880: F, t20885: F, t20889: F, t20892: F, t20894: F, t20898: F, t20900: F, t20902: F, t20908: F, t20910: F, t20912: F, t20917: F, t20923: F, t3961: F, t3964: F, t7043: F) -> (F, F) {
    let t20927 = t1468 * t20926;
    let t20928 = t1464 * t20927;
    let t20930 = -0.16581944444444444444e-2 * t20875 + 0.11054629629629629629e-2 * t20880 - 0.33163888888888888888e-2 * t20885 + 0.27636574074074074073e-2 * t20889 + 0.11054629629629629629e-2 * t15800 + 0.22109259259259259259e-2 * t20892 - 0.22109259259259259259e-2 * t20894 - 0.13345e0 * t16744 * t1944 - 0.33163888888888888888e-2 * t20898 - 0.58958024691358024689e-2 * t20900 + 0.11054629629629629629e-2 * t20902 - 0.7369753086419753086e-3 * t15826 + 0.11054629629629629629e-2 * t20908 - 0.33163888888888888888e-2 * t20910 + 0.22109259259259259259e-2 * t20912 + 0.66725e-1 * t3964 * t7043 + 0.66725e-1 * t1364 * t20917 + 0.890445125e-2 * t3961 * t20917 - 0.13265555555555555555e-1 * t20923 - 0.16581944444444444444e-2 * t20928;
    (t20928, t20930)
}
