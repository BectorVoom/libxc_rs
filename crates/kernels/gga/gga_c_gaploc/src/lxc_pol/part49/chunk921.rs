//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 921/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk921<F: Float>(t40182: F, t40184: F, t40187: F, t12865: F, t1580: F, t31828: F, t874: F, t1445: F, t597: F, t10151: F, t2293: F, t10557: F, t9324: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41852 = F::new(0.25561950635947166451e0) * t40182;
    let t41853 = F::new(0.89376224879626066674e-1) * t40184;
    let t41854 = F::new(0.17875244975925213335e0) * t40187;
    let t41863 = t1580 * t12865;
    let t41865 = t31828 * t874;
    let t41867 = t597 * t1445 * t41865;
    let t41869 = t10151 * t2293;
    let t41871 = t597 * t1445 * t41869;
    let t41874 = F::new(0.85801175884441024006e1) * t10557 * t9324;
    (t41852, t41853, t41854, t41863, t41865, t41867, t41869, t41871, t41874)
}
