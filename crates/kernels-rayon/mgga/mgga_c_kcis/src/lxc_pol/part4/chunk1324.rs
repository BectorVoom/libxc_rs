//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1324/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1324(t12263: f64, t12271: f64, t12275: f64, t12277: f64, t12279: f64, t12286: f64, t12303: f64, t12306: f64, t12307: f64, t1364: f64, t16805: f64, t16806: f64, t16809: f64, t16812: f64, t16820: f64, t16824: f64, t16992: f64, t16997: f64, t17000: f64, t3961: f64, t3964: f64, t5738: f64, t5886: f64) -> f64 {
    let t17004 = -t16805 + 0.22109259259259259258e-2_f64 * t16806 - t16809 + 0.99491666666666666664e-2_f64 * t16812 - 0.3684876543209876543e-3_f64 * t12263 + 0.33163888888888888888e-2_f64 * t12271 - 0.73697530864197530861e-3_f64 * t12275 + 0.11054629629629629629e-2_f64 * t12277 + 0.11054629629629629629e-2_f64 * t12279 - 0.11054629629629629629e-2_f64 * t12303 - 0.55273148148148148147e-3_f64 * t16820 + t12306 + 0.16581944444444444444e-2_f64 * t12307 - 0.2671335375e-1_f64 * t3961 * t16824 + 0.178089025e-1_f64 * t12286 * t5886 - 0.66725e-1_f64 * t1364 * t16992 + 0.33163888888888888888e-2_f64 * t16997 - 0.24872916666666666666e-2_f64 * t17000 - 0.13345e0_f64 * t3964 * t5738;
    t17004
}
