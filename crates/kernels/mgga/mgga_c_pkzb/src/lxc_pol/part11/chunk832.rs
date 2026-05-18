//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 832/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk832<F: Float>(t3396: F, t614: F, t568: F, t596: F, t8817: F, t1029: F, t1031: F, t160: F, t162: F, t2625: F, t2631: F, t2633: F, t2636: F, t3431: F, t3435: F, t3438: F, t594: F, t597: F, t8859: F, t8865: F, t8873: F, t8876: F) -> (F, F, F) {
    let t8881 = t614 * t3396;
    let t8882 = t8881 * t568;
    let t8885 = t596 * t8817;
    let t8888 = F::new(6.0) * t1029 * t2636 + F::new(6.0) * t1031 * t2625 + F::new(3.0) * t160 * t8885 - t162 * t8859 + F::new(60.0) * t2631 * t8873 - F::new(24.0) * t2631 * t8876 - F::new(12.0) * t2631 * t8882 - F::new(24.0) * t2633 * t8865 + F::new(3.0) * t3431 * t597 - F::new(12.0) * t3435 * t594 + F::new(3.0) * t3438 * t594;
    (t8882, t8885, t8888)
}
