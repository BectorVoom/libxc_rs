//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1238/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1238(t1396: f64, t15973: f64, t11826: f64, t1464: f64, t4012: f64, t5885: f64, t2007: f64, t3245: f64, t11882: f64, t11884: f64, t15927: f64, t15932: f64, t15934: f64, t15939: f64, t15942: f64, t15944: f64, t15947: f64, t15950: f64, t15953: f64, t15958: f64, t15961: f64, t15964: f64, t15968: f64, t15971: f64, t3961: f64) -> (f64, f64, f64, f64) {
    let t15974 = t1396 * t15973;
    let t15975 = t11826 * t15974;
    let t15976 = t1464 * t15975;
    let t15978 = t5885 * t4012;
    let t15983 = t3245 * t2007;
    let t15985 = -0.22109259259259259258e-2_f64 * t15927 + 0.73697530864197530862e-3_f64 * t15932 - 0.22109259259259259258e-2_f64 * t15934 + 0.49745833333333333332e-2_f64 * t15939 + t15942 + 0.16581944444444444444e-2_f64 * t15944 + 0.27636574074074074073e-2_f64 * t15947 + 0.16581944444444444444e-2_f64 * t15950 - 0.66327777777777777776e-2_f64 * t15953 + 0.66327777777777777776e-2_f64 * t15958 - 0.33163888888888888888e-2_f64 * t15961 - 0.11054629629629629629e-2_f64 * t15964 + 0.55273148148148148147e-2_f64 * t15968 - 0.33163888888888888888e-2_f64 * t15971 - 0.1492375e-1_f64 * t15976 + 0.890445125e-2_f64 * t3961 * t15978 - 0.73697530864197530861e-3_f64 * t11882 + 0.22109259259259259258e-2_f64 * t11884 + 0.14739506172839506172e-2_f64 * t15983;
    (t15976, t15978, t15983, t15985)
}
