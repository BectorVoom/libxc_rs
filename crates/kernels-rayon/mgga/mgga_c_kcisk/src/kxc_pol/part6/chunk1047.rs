//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1047/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1047(t1340: f64, t31175: f64, t1411: f64, t14208: f64, t30489: f64, t2232: f64, t25351: f64, t1220: f64, t13437: f64, t19948: f64, t20886: f64, t2174: f64, t26914: f64, t26919: f64, t26936: f64, t27008: f64, t27016: f64, t27037: f64, t30775: f64, t31153: f64, t31168: f64, t31173: f64, t6221: f64, t8060: f64, t8064: f64) -> (f64, f64, f64, f64) {
    let t31176 = t1340 * t31175;
    let t31177 = t1411 * t31176;
    let t31179 = t14208 * t30489;
    let t31180 = t1340 * t31179;
    let t31181 = t1411 * t31180;
    let t31183 = t25351 * t2232;
    let t31184 = t1411 * t31183;
    let t31194 = -0.99491666666666666664e-2_f64 * t31153 - 0.11054629629629629629e-2_f64 * t19948 - 0.11054629629629629629e-2_f64 * t26914 + 0.66327777777777777776e-2_f64 * t26919 + 0.49745833333333333332e-2_f64 * t26936 + 0.223494e0_f64 * t20886 * t8064 - 0.579e0_f64 * t27016 * t2174 - 0.43134342e-1_f64 * t13437 * t30775 + 0.99491666666666666664e-2_f64 * t31168 - 0.49745833333333333332e-2_f64 * t31173 - 0.16581944444444444444e-2_f64 * t31177 + 0.73697530864197530862e-3_f64 * t31181 - 0.74618749999999999998e-2_f64 * t31184 + 0.579e0_f64 * t6221 * t8064 - 0.386e0_f64 * t1220 * t30775 - 0.579e0_f64 * t6221 * t8060 + 0.55273148148148148145e-2_f64 * t27008 + 0.44218518518518518516e-2_f64 * t27037;
    (t31177, t31181, t31184, t31194)
}
