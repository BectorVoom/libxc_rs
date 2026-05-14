//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1008/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1008<F: Float>(t10023: F, t28889: F, t2021: F, t7339: F, t7372: F, t3296: F, t6100: F, t21451: F, t2365: F, t6111: F, t1967: F, t21455: F, t7810: F, t883: F, t21460: F, t20671: F, t22538: F, t22984: F) -> (F, F, F, F, F, F, F) {
    let t28891 = 0.3575048995185042667e0 * t10023 * t28889;
    let t28915 = 0.59584149919750711116e-1 * t2021 * t7339 * t7372;
    let t28916 = t6100 * t3296;
    let t28920 = 0.11916829983950142223e0 * t6111 * t2365 * t21451;
    let t28936 = t7810 * t1967 * t883 * t21455;
    let t28940 = t7810 * t1967 * t883 * t21460;
    let t28944 = 0.17041300423964777634e0 * t22538 * t20671 * t22984;
    (t28891, t28915, t28916, t28920, t28936, t28940, t28944)
}
