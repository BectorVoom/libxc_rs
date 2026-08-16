//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1112/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1112<F: Float>(t28936: F, t1967: F, t21460: F, t7810: F, t883: F, t20671: F, t22538: F, t22984: F, t23183: F, t7391: F, t1457: F, t7722: F) -> (F, F, F, F, F) {
    let t28937 = F::cast_from(0.76685851907841499352e0_f64) * t28936;
    let t28940 = t7810 * t1967 * t883 * t21460;
    let t28941 = F::cast_from(0.38342925953920749676e0_f64) * t28940;
    let t28944 = F::cast_from(0.17041300423964777634e0_f64) * t22538 * t20671 * t22984;
    let t28946 = F::cast_from(0.17875244975925213335e0_f64) * t23183 * t7391;
    let t28976 = t1457 * t7722;
    (t28937, t28941, t28944, t28946, t28976)
}
