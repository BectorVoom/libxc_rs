//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1255/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1255<F: Float>(t12933: F, t12940: F, t1629: F, t1636: F, t17710: F, t18268: F, t2128: F, t23253: F, t23255: F, t23265: F, t23268: F, t23272: F, t23373: F, t4475: F, t4480: F, t6222: F, t6225: F, t6256: F, t633: F, t7537: F, t7566: F) -> (F,) {
    let t23375 = 2.0 * t12933 * t7537 - 6.0 * t12940 * t23265 - t1629 * t23373 - t1636 * t23255 - 2.0 * t17710 * t2128 + 4.0 * t18268 * t6225 + t23253 * t633 + 4.0 * t23268 * t4480 + 2.0 * t23272 * t4480 - t4475 * t7566 - 2.0 * t6222 * t6256;
    (t23375,)
}
