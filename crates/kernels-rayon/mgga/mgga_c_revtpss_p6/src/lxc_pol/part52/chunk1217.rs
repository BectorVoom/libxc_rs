//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1217/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1217(t34097: f64, t890: f64, t1940: f64, t2255: f64, t8657: f64, t102851: f64, t110165: f64, t121751: f64, t125962: f64, t126031: f64, t1468: f64, t26425: f64, t27376: f64, t27391: f64, t28460: f64, t28472: f64, t31873: f64, t32487: f64, t32491: f64, t32498: f64, t32506: f64, t34098: f64, t98658: f64, t98785: f64) -> (f64, f64, f64) {
    let t127914 = t34097 * t890;
    let t127929 = t1940 * t8657 * t2255;
    let t127939 = -3.0_f64 * t28472 * t98785 * t127914 - 3.0_f64 / 2.0_f64 * t26425 * t126031 + t110165 * t32506 + t1940 * t32487 * t1468 / 2.0_f64 + t102851 * t34098 - t1940 * t32491 * t27391 / 2.0_f64 + t127929 + t28472 * t125962 - 3.0_f64 / 2.0_f64 * t26425 * t98658 * t32498 - t1940 * t28460 * t31873 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t121751 * t27376;
    (t127914, t127929, t127939)
}
