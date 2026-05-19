//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1228/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1228<F: Float>(t3622: F, t7560: F, t20637: F, t2852: F, t30231: F, t3626: F, t2875: F, t9242: F, t10767: F, t204: F, t648: F) -> (F, F, F, F, F) {
    let t30270 = F::cast_from(0.17544670867903938621e1_f64) * t7560 * t3622;
    let t30273 = F::cast_from(0.31168546390226634766e3_f64) * t20637 * t2852 * t30231;
    let t30275 = F::cast_from(0.51947577317044391276e2_f64) * t7560 * t3626;
    let t30277 = F::cast_from(0.51947577317044391276e2_f64) * t9242 * t2875;
    let t30284 = t204 * t648 * t10767;
    (t30270, t30273, t30275, t30277, t30284)
}
