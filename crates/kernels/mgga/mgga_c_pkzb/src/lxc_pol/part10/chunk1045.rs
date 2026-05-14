//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1045/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1045<F: Float>(t1029: F, t1031: F, t160: F, t162: F, t2625: F, t2631: F, t2633: F, t2636: F, t3431: F, t3435: F, t3438: F, t594: F, t597: F, t8859: F, t8865: F, t8873: F, t8876: F, t8882: F, t8885: F) -> (F,) {
    let t8888 = 6.0 * t1029 * t2636 + 6.0 * t1031 * t2625 + 3.0 * t160 * t8885 - t162 * t8859 + 60.0 * t2631 * t8873 - 24.0 * t2631 * t8876 - 12.0 * t2631 * t8882 - 24.0 * t2633 * t8865 + 3.0 * t3431 * t597 - 12.0 * t3435 * t594 + 3.0 * t3438 * t594;
    (t8888,)
}
