//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1062/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1062<F: Float>(t1505: F, t5895: F, t2016: F, t4188: F, t12321: F, t41: F, t4291: F, t5747: F, t2033: F, t4121: F, t492: F, t6015: F) -> (F, F, F, F, F, F) {
    let t17308 = t5895 * t1505;
    let t17311 = t2016 * t4188;
    let t17382 = t41 * t12321;
    let t17391 = t5747 * t4291;
    let t17396 = t2033 * t4121;
    let t17412 = t6015 * t492;
    (t17308, t17311, t17382, t17391, t17396, t17412)
}
