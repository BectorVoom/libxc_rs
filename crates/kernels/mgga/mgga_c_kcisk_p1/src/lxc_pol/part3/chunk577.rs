//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 577/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk577<F: Float>(t1224: F, t4640: F, t4840: F, t1697: F, t4644: F, t4648: F, t4835: F, t4838: F, t1701: F, t1705: F, t1704: F, t617: F) -> (F, F, F, F, F, F) {
    let t4842 = t1224 * t4840 * t4640;
    let t4845 = t1224 * t1697 * t4644;
    let t4848 = t1224 * t1697 * t4648;
    let t4850 = t4835 + F::cast_from(0.11872222222222222222e-1_f64) * t4838 - F::cast_from(0.11872222222222222222e-1_f64) * t4842 + F::cast_from(0.35616666666666666666e-1_f64) * t4845 - F::cast_from(0.17808333333333333333e-1_f64) * t4848;
    let t4853 = t1701 * t1705;
    let t4856 = t1704 * t617;
    (t4842, t4845, t4848, t4850, t4853, t4856)
}
