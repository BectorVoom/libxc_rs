//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 873/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk873<F: Float>(t36632: F, t959: F, t2660: F, t36512: F, t10867: F, t10972: F, t1457: F, t2684: F, t45423: F, t7585: F, t3651: F, t9972: F) -> (F, F, F, F, F) {
    let t45574 = t36632 * t959;
    let t45575 = F::cast_from(0.14896037479937677779e-1_f64) * t45574;
    let t45577 = F::cast_from(0.25025342966295298669e1_f64) * t36512 * t2660;
    let t45580 = F::cast_from(0.50050685932590597338e1_f64) * t10867 * t1457 * t10972;
    let t45586 = F::cast_from(0.43710935587469654631e2_f64) * t2684 * t7585 * t45423;
    let t45588 = F::cast_from(0.25025342966295298669e1_f64) * t3651 * t9972;
    (t45575, t45577, t45580, t45586, t45588)
}
