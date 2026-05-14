//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1246/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1246<F: Float>(t2294: F, t2598: F, t9508: F, t6132: F, t8821: F, t2133: F, t8795: F, t259: F, t9325: F, t546: F, t565: F, t7494: F, t8853: F, t2139: F, t9261: F, t9135: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27831 = t2598 * t2294 * t9508;
    let t27834 = t6132 * t2294 * t8821;
    let t27858 = t2133 * t2294 * t8795;
    let t27866 = t9325 * t259;
    let t27867 = t546 * t27866;
    let t27870 = t565 * t27866;
    let t27885 = t7494 * t8853;
    let t27899 = t2139 * t2294 * t9261;
    let t27910 = t2139 * t2294 * t9135;
    (t27831, t27834, t27858, t27866, t27867, t27870, t27885, t27899, t27910)
}
