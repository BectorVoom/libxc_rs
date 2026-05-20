//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2041/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2041<F: Float>(t102854: F, t102888: F, t107892: F, t107908: F, t107927: F, t107934: F, t107958: F, t107970: F, t110699: F, t110704: F, t1940: F, t2071: F, t2403: F, t26425: F, t26585: F, t26590: F, t27764: F, t27770: F, t27802: F, t27806: F, t28291: F, t28460: F, t29939: F, t29970: F, t30471: F, t33: F, t4541: F, t50080: F, t7428: F, t7869: F) -> F {
    let t110989 = -t1940 * t28460 * t27802 + F::new(6.0) * t110704 * t27764 + F::new(3.0) * t50080 * t30471 - F::new(3.0) * t26425 * t107892 - t1940 * t102854 * t7869 - F::new(6.0) * t28291 * t107927 + F::new(3.0) * t26425 * t107908 - F::new(3.0) * t102888 * t27770 + F::new(6.0) * t28291 * t107934 - t1940 * t28460 * t27806 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t107970 + F::new(3.0) * t4541 * t7428 * t29939 + t1940 * t26590 * t107958 + t1940 * t110699 * t33 / F::new(2.0) - t1940 * t26585 * t29970 / F::new(2.0);
    t110989
}
