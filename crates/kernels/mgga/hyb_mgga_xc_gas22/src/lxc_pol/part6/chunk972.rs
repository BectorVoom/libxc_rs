//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 972/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk972<F: Float>(t132: F, t338: F, t8949: F, t3452: F, t930: F, t1386: F, t2447: F, t8589: F, t1433: F, t7108: F, t2602: F, t2579: F, t3604: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t8950 = t8949 * t338;
    let t8951 = t3452 * t930;
    let t8953 = t1386 * t2447;
    let t8955 = piecewise3::<F>(t133, F::new(0.0), -t8589);
    let t8964 = t7108 * t1433;
    let t8965 = t8964 * t2602;
    let t8968 = t3604 * t2579;
    (t8950, t8951, t8953, t8955, t8965, t8968)
}
