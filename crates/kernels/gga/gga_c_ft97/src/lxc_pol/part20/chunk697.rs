//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 697/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk697<F: Float>(t15010: F, t15055: F, t845: F, t91: F, t2755: F, t4226: F, t856: F, t2789: F, t4191: F, t10631: F, t1234: F, t2756: F, t10649: F, t10797: F, t14718: F, t14892: F, t14899: F, t14949: F, t14951: F) -> (F, F, F, F, F, F) {
    let t15056 = t15010 + t15055;
    let t15058 = t91 * t845 * t15056;
    let t15060 = t2755 * t4226;
    let t15062 = t91 * t15060 * t856;
    let t15065 = t91 * t4191 * t2789;
    let t15069 = t91 * t10631 * t1234 * t2756;
    let t15071 = -22.0 / 9.0 * t14718 - t10649 - t14892 - t14949 + 2.0 / 3.0 * t14899 + t14951 + t15058 / 2.0 - t10797 - t15062 / 2.0 - t15065 / 4.0 + 3.0 / 8.0 * t15069;
    (t15056, t15058, t15062, t15065, t15069, t15071)
}
