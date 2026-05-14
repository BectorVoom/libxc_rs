//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1037/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1037<F: Float>(t2492: F, t6907: F, t9267: F, t4811: F, t9538: F, t6895: F, t4781: F, t9274: F, t1645: F, t6474: F, t1423: F, t2326: F, t9561: F, t1397: F, t6851: F, t9562: F) -> (F, F, F, F, F, F, F, F) {
    let t31129 = t9267 * t2492 * t6907;
    let t31131 = t4811 * t9538;
    let t31135 = 0.19171462976960374838e1 * t9267 * t2492 * t6895;
    let t31144 = t4781 * t9274;
    let t31153 = t1645 * t6474;
    let t31158 = t1423 * t2326;
    let t31160 = 0.3575048995185042667e0 * t9561 * t31158;
    let t31163 = 0.17875244975925213335e0 * t1397 * t6851 * t9562;
    (t31129, t31131, t31135, t31144, t31153, t31158, t31160, t31163)
}
