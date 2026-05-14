//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1006/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1006<F: Float>(t28916: F, t21451: F, t2365: F, t6111: F, t1967: F, t21455: F, t7810: F, t883: F, t21460: F, t20671: F, t22538: F, t22984: F, t23183: F, t7391: F, t1457: F, t7722: F) -> (F, F, F, F, F, F, F) {
    let t28917 = 0.38342925953920749676e0 * t28916;
    let t28920 = 0.11916829983950142223e0 * t6111 * t2365 * t21451;
    let t28936 = t7810 * t1967 * t883 * t21455;
    let t28937 = 0.76685851907841499352e0 * t28936;
    let t28940 = t7810 * t1967 * t883 * t21460;
    let t28941 = 0.38342925953920749676e0 * t28940;
    let t28944 = 0.17041300423964777634e0 * t22538 * t20671 * t22984;
    let t28946 = 0.17875244975925213335e0 * t23183 * t7391;
    let t28976 = t1457 * t7722;
    (t28917, t28920, t28937, t28941, t28944, t28946, t28976)
}
