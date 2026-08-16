//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1251/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1251(t1385: f64, t20916: f64, t2006: f64, t5871: f64, t303: f64, t1497: f64, t7257: f64, t1495: f64, t1468: f64, t1464: f64, t1364: f64, t15800: f64, t15826: f64, t16744: f64, t1944: f64, t20875: f64, t20880: f64, t20885: f64, t20889: f64, t20892: f64, t20894: f64, t20898: f64, t20900: f64, t20902: f64, t20908: f64, t20910: f64, t20912: f64, t3961: f64, t3964: f64, t7043: f64) -> (f64, f64, f64, f64) {
    let t20917 = t20916 * t1385;
    let t20922 = t5871 * t2006;
    let t20923 = t303 * t20922;
    let t20925 = t7257 * t1497;
    let t20926 = t1495 * t20925;
    let t20927 = t1468 * t20926;
    let t20928 = t1464 * t20927;
    let t20930 = -0.16581944444444444444e-2_f64 * t20875 + 0.11054629629629629629e-2_f64 * t20880 - 0.33163888888888888888e-2_f64 * t20885 + 0.27636574074074074073e-2_f64 * t20889 + 0.11054629629629629629e-2_f64 * t15800 + 0.22109259259259259259e-2_f64 * t20892 - 0.22109259259259259259e-2_f64 * t20894 - 0.13345e0_f64 * t16744 * t1944 - 0.33163888888888888888e-2_f64 * t20898 - 0.58958024691358024689e-2_f64 * t20900 + 0.11054629629629629629e-2_f64 * t20902 - 0.7369753086419753086e-3_f64 * t15826 + 0.11054629629629629629e-2_f64 * t20908 - 0.33163888888888888888e-2_f64 * t20910 + 0.22109259259259259259e-2_f64 * t20912 + 0.66725e-1_f64 * t3964 * t7043 + 0.66725e-1_f64 * t1364 * t20917 + 0.890445125e-2_f64 * t3961 * t20917 - 0.13265555555555555555e-1_f64 * t20923 - 0.16581944444444444444e-2_f64 * t20928;
    (t20923, t20925, t20928, t20930)
}
