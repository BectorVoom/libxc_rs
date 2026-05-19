//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 579/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk579<F: Float>(t3005: F, t3006: F, t971: F, t2917: F, t2966: F, t2919: F, t2922: F, t2925: F, t2928: F, t2945: F, t2953: F, t2961: F, t2963: F, t2968: F, t2972: F, t2975: F, t2978: F) -> (F, F, F, F) {
    let t3008 = t3005 * t3006 * t971;
    let t3013 = F::cast_from(0.40256666666666666667e0_f64) * t2917;
    let t3020 = F::new(0.137975e0) * t2966;
    let t3025 = -F::new(0.1294625e1) * t2945 + F::new(0.258925e1) * t2953 + t3013 + F::cast_from(0.20128333333333333334e0_f64) * t2919 - F::cast_from(0.20128333333333333333e0_f64) * t2922 + F::new(0.60385e0) * t2925 - F::new(0.301925e0) * t2928 + F::new(0.82524375e-1) * t2961 + F::new(0.16504875e0) * t2963 + t3020 + F::new(0.11038e0) * t2968 - F::new(0.27595e-1) * t2972 + F::new(0.16557e0) * t2975 - F::new(0.82785e-1) * t2978;
    (t3008, t3013, t3020, t3025)
}
