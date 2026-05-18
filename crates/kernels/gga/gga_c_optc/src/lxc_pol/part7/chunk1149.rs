//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1149/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1149<F: Float>(t23682: F, t23685: F, t23873: F, t780: F, t2383: F, t2391: F, t7512: F, t7516: F, t7552: F, t7557: F, t7519: F, t23660: F, t23664: F, t23667: F, t23670: F, t23673: F, t23676: F, t23679: F) -> (F, F, F, F, F, F) {
    let t23926 = F::new(0.31310740740740740741e1) * t23682;
    let t23927 = F::new(0.13490888888888888889e1) * t23685;
    let t23928 = t780 * t23873;
    let t23931 = t7512 * t2383 * t2391;
    let t23933 = t7516 * t7552;
    let t23936 = t7557 * t2383 * t2391;
    let t23938 = t7519 * t7552;
    let t23940 = F::new(0.24154e1) * t23660 - F::new(0.298026e1) * t23664 + F::new(0.66228e0) * t23667 + F::new(0.72462e1) * t23670 - F::new(0.80513333333333333332e0) * t23673 - F::new(0.20128333333333333334e1) * t23676 - F::new(0.108693e2) * t23679 + t23926 + t23927 + F::new(0.258925e1) * t23928 + F::new(0.11651625e2) * t23931 - F::new(0.51785e1) * t23933 - F::new(0.247573125e0) * t23936 + F::new(0.3300975e0) * t23938;
    (t23928, t23931, t23933, t23936, t23938, t23940)
}
