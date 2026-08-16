//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 913/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk913<F: Float>(t12263: F, t12271: F, t12275: F, t12277: F, t12279: F, t12286: F, t12303: F, t12306: F, t12307: F, t1364: F, t16805: F, t16806: F, t16809: F, t16812: F, t16820: F, t16824: F, t16992: F, t16997: F, t17000: F, t3961: F, t3964: F, t5738: F, t5886: F) -> F {
    let t17004 = -t16805 + F::cast_from(0.22109259259259259258e-2_f64) * t16806 - t16809 + F::cast_from(0.99491666666666666664e-2_f64) * t16812 - F::cast_from(0.3684876543209876543e-3_f64) * t12263 + F::cast_from(0.33163888888888888888e-2_f64) * t12271 - F::cast_from(0.73697530864197530861e-3_f64) * t12275 + F::cast_from(0.11054629629629629629e-2_f64) * t12277 + F::cast_from(0.11054629629629629629e-2_f64) * t12279 - F::cast_from(0.11054629629629629629e-2_f64) * t12303 - F::cast_from(0.55273148148148148147e-3_f64) * t16820 + t12306 + F::cast_from(0.16581944444444444444e-2_f64) * t12307 - F::cast_from(0.2671335375e-1_f64) * t3961 * t16824 + F::cast_from(0.178089025e-1_f64) * t12286 * t5886 - F::cast_from(0.66725e-1_f64) * t1364 * t16992 + F::cast_from(0.33163888888888888888e-2_f64) * t16997 - F::cast_from(0.24872916666666666666e-2_f64) * t17000 - F::cast_from(0.13345e0_f64) * t3964 * t5738;
    t17004
}
