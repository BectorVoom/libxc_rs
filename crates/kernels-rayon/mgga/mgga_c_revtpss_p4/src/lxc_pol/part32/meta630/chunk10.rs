//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2041/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2041(t102854: f64, t102888: f64, t107892: f64, t107908: f64, t107927: f64, t107934: f64, t107958: f64, t107970: f64, t110699: f64, t110704: f64, t1940: f64, t2071: f64, t2403: f64, t26425: f64, t26585: f64, t26590: f64, t27764: f64, t27770: f64, t27802: f64, t27806: f64, t28291: f64, t28460: f64, t29939: f64, t29970: f64, t30471: f64, t33: f64, t4541: f64, t50080: f64, t7428: f64, t7869: f64) -> f64 {
    let t110989 = -t1940 * t28460 * t27802 + 6.0_f64 * t110704 * t27764 + 3.0_f64 * t50080 * t30471 - 3.0_f64 * t26425 * t107892 - t1940 * t102854 * t7869 - 6.0_f64 * t28291 * t107927 + 3.0_f64 * t26425 * t107908 - 3.0_f64 * t102888 * t27770 + 6.0_f64 * t28291 * t107934 - t1940 * t28460 * t27806 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t107970 + 3.0_f64 * t4541 * t7428 * t29939 + t1940 * t26590 * t107958 + t1940 * t110699 * t33 / 2.0_f64 - t1940 * t26585 * t29970 / 2.0_f64;
    t110989
}
