//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 883/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk883<F: Float>(t1544: F, t1583: F, t18875: F, t1940: F, t198: F, t207: F, t2071: F, t2403: F, t26585: F, t26590: F, t27375: F, t27384: F, t28455: F, t28460: F, t4343: F, t4433: F, t4537: F, t4541: F, t7428: F, t7432: F, t775: F, t8020: F, t890: F, t892: F) -> (F,) {
    let t28522 = t198 * t207 * t28455 * t892 + 3.0 * t1544 * t2403 * t7428 - t1583 * t1940 * t26585 - 3.0 * t18875 * t2403 * t7432 + 2.0 * t1940 * t26590 * t27384 - t1940 * t28460 * t890 - t1940 * t4537 * t7432 + 3.0 * t2071 * t2403 * t4343 + 6.0 * t2071 * t4433 * t4541 - 3.0 * t2403 * t27375 * t7432 + 3.0 * t2403 * t775 * t8020;
    (t28522,)
}
