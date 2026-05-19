//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1043/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1043<F: Float>(t13597: F, t762: F, t1450: F, t5778: F, t2516: F, t5571: F, t5566: F, t72: F, t757: F, t1320: F, t5567: F, t5569: F) -> (F, F, F, F, F, F) {
    let t13599 = F::cast_from(0.11696447245269292414e1_f64) * t13597 * t762;
    let t13600 = t5778 * t1450;
    let t13611 = t5571 * t2516;
    let t13613 = t5566 * t72;
    let t13615 = F::cast_from(0.36622894612013090108e-3_f64) * t13613 * t757;
    let t13620 = F::new(8.0) * t1320 * t5567;
    let t13621 = t1320 * t5569;
    (t13599, t13600, t13611, t13615, t13620, t13621)
}
