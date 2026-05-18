//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 961/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk961<F: Float>(t9538: F, t329: F, t64: F, t358: F, t283: F, t1135: F, t9528: F, t2817: F, t2861: F, t2822: F, t2857: F, t1018: F, t86: F, t9526: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t9539 = t9538 * sigma0;
    let t9543 = t64 * t329;
    let t9545 = F::new(1.0) / t358 / t9543;
    let t9546 = t283 * t9545;
    let t9552 = t9528 * t1135;
    let t9557 = t2861 * t2817;
    let t9559 = t2822 * t2857;
    let t9562 = t86 * t9526 * t1018;
    (t9539, t9545, t9546, t9552, t9557, t9559, t9562)
}
