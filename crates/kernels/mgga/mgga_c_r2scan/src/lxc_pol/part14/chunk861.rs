//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 861/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk861<F: Float>(t10803: F, t2124: F, t5115: F, t3295: F, t502: F, t550: F) -> (F, F, F, F) {
    let t10804 = 0.10975748638225852664e-1 * t10803;
    let t10805 = t2124 * t5115;
    let t10806 = t3295 * t10805;
    let t10810 = t550 * t502;
    (t10804, t10805, t10806, t10810)
}
