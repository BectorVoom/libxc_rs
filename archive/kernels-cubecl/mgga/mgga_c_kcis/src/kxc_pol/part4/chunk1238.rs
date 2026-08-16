//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1238/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1238<F: Float>(t1396: F, t15973: F, t11826: F, t1464: F, t4012: F, t5885: F, t2007: F, t3245: F, t11882: F, t11884: F, t15927: F, t15932: F, t15934: F, t15939: F, t15942: F, t15944: F, t15947: F, t15950: F, t15953: F, t15958: F, t15961: F, t15964: F, t15968: F, t15971: F, t3961: F) -> (F, F, F, F) {
    let t15974 = t1396 * t15973;
    let t15975 = t11826 * t15974;
    let t15976 = t1464 * t15975;
    let t15978 = t5885 * t4012;
    let t15983 = t3245 * t2007;
    let t15985 = -F::cast_from(0.22109259259259259258e-2_f64) * t15927 + F::cast_from(0.73697530864197530862e-3_f64) * t15932 - F::cast_from(0.22109259259259259258e-2_f64) * t15934 + F::cast_from(0.49745833333333333332e-2_f64) * t15939 + t15942 + F::cast_from(0.16581944444444444444e-2_f64) * t15944 + F::cast_from(0.27636574074074074073e-2_f64) * t15947 + F::cast_from(0.16581944444444444444e-2_f64) * t15950 - F::cast_from(0.66327777777777777776e-2_f64) * t15953 + F::cast_from(0.66327777777777777776e-2_f64) * t15958 - F::cast_from(0.33163888888888888888e-2_f64) * t15961 - F::cast_from(0.11054629629629629629e-2_f64) * t15964 + F::cast_from(0.55273148148148148147e-2_f64) * t15968 - F::cast_from(0.33163888888888888888e-2_f64) * t15971 - F::cast_from(0.1492375e-1_f64) * t15976 + F::cast_from(0.890445125e-2_f64) * t3961 * t15978 - F::cast_from(0.73697530864197530861e-3_f64) * t11882 + F::cast_from(0.22109259259259259258e-2_f64) * t11884 + F::cast_from(0.14739506172839506172e-2_f64) * t15983;
    (t15976, t15978, t15983, t15985)
}
