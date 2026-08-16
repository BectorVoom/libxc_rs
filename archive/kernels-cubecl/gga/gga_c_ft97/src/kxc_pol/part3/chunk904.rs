//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 904/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk904<F: Float>(t1127: F, t689: F, t2427: F, t25: F, t677: F, t200: F, t709: F, t13473: F, t3758: F, t1113: F, t237: F, t213: F, t5001: F) -> (F, F, F, F, F, F, F) {
    let t17950 = t689 * t1127;
    let t17957 = t2427 * t25;
    let t17958 = t677 * t17957;
    let t17959 = t200 * t1127;
    let t17960 = t17959 * t709;
    let t17964 = t3758 * t13473;
    let t17965 = t200 * t1113;
    let t17966 = t17965 * t709;
    let t17970 = t237 * t25;
    let t17971 = t3758 * t17970;
    let t17975 = t213 * t5001;
    (t17950, t17958, t17960, t17964, t17966, t17971, t17975)
}
