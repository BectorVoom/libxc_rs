//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1047/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1047<F: Float>(t1340: F, t31175: F, t1411: F, t14208: F, t30489: F, t2232: F, t25351: F, t1220: F, t13437: F, t19948: F, t20886: F, t2174: F, t26914: F, t26919: F, t26936: F, t27008: F, t27016: F, t27037: F, t30775: F, t31153: F, t31168: F, t31173: F, t6221: F, t8060: F, t8064: F) -> (F, F, F, F) {
    let t31176 = t1340 * t31175;
    let t31177 = t1411 * t31176;
    let t31179 = t14208 * t30489;
    let t31180 = t1340 * t31179;
    let t31181 = t1411 * t31180;
    let t31183 = t25351 * t2232;
    let t31184 = t1411 * t31183;
    let t31194 = -F::new(0.99491666666666666664e-2) * t31153 - F::new(0.11054629629629629629e-2) * t19948 - F::new(0.11054629629629629629e-2) * t26914 + F::new(0.66327777777777777776e-2) * t26919 + F::new(0.49745833333333333332e-2) * t26936 + F::new(0.223494e0) * t20886 * t8064 - F::new(0.579e0) * t27016 * t2174 - F::new(0.43134342e-1) * t13437 * t30775 + F::new(0.99491666666666666664e-2) * t31168 - F::new(0.49745833333333333332e-2) * t31173 - F::new(0.16581944444444444444e-2) * t31177 + F::new(0.73697530864197530862e-3) * t31181 - F::new(0.74618749999999999998e-2) * t31184 + F::new(0.579e0) * t6221 * t8064 - F::new(0.386e0) * t1220 * t30775 - F::new(0.579e0) * t6221 * t8060 + F::new(0.55273148148148148145e-2) * t27008 + F::new(0.44218518518518518516e-2) * t27037;
    (t31177, t31181, t31184, t31194)
}
