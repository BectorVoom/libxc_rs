//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 857/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk857<F: Float>(t4827: F, t4839: F, t4842: F, t5000: F, t5004: F, t5008: F, t5020: F, t6010: F, t6012: F, t7025: F, t8641: F, t3128: F, t424: F) -> (F, F) {
    let t9055 = t5000 + t5004 + t5008 + t4827 - t4839 - t8641 + t5020 + t6010 - F::new(0.571528e-1) * t6012 - t4842 + t7025;
    let t9056 = t424 * t3128;
    (t9055, t9056)
}
