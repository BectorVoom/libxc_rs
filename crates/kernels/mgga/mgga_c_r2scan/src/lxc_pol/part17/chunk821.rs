//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 821/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk821<F: Float>(t5015: F, t7031: F, t7032: F, t7051: F, t4827: F, t4839: F, t4842: F, t4845: F, t4996: F, t5000: F, t5004: F, t5008: F, t5020: F, t7025: F, t7036: F, t7095: F) -> (F, F, F, F, F) {
    let t8641 = F::new(0.24415263074675393405e-3) * t5015;
    let t8642 = F::new(2.0) * t7031;
    let t8643 = F::new(0.48830526149350786811e-3) * t7032;
    let t8644 = F::new(16.0) * t7051;
    let t8645 = t4996 - t5000 - t5004 - t5008 - t4827 + t4839 + t8641 - t5020 + t4842 - t7025 + t8642 + t8643 - t7036 - t4845 - t8644 - t7095;
    (t8641, t8642, t8643, t8644, t8645)
}
