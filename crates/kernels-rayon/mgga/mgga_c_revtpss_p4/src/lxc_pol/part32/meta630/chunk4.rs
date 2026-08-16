//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2035/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2035(t106554: f64, t106565: f64, t106610: f64, t107793: f64, t107805: f64, t110698: f64, t18435: f64, t18498: f64, t18838: f64, t1940: f64, t198: f64, t207: f64, t2071: f64, t2403: f64, t26425: f64, t26585: f64, t26590: f64, t27375: f64, t28291: f64, t28460: f64, t30420: f64, t4541: f64, t5962: f64, t6075: f64, t7428: f64, t7432: f64, t77408: f64, t77425: f64, t77441: f64, t775: f64, t892: f64, t95964: f64) -> f64 {
    let t110792 = -t1940 * t26585 * t6075 + 3.0_f64 * t2403 * t30420 * t775 + 3.0_f64 * t2403 * t7428 * t5962 + 12.0_f64 * t4541 * t2071 * t18498 - t1940 * t7432 * t18838 - 3.0_f64 * t2403 * t7432 * t77425 + 2.0_f64 * t1940 * t26590 * t106610 + 6.0_f64 * t4541 * t2071 * t18435 - 6.0_f64 * t2403 * t7432 * t77441 + 4.0_f64 * t1940 * t26590 * t106554 - 6.0_f64 * t4541 * t7432 * t77408 - 6.0_f64 * t2403 * t28460 * t27375 + t198 * t207 * t110698 * t892 - 12.0_f64 * t28291 * t107793 + 12.0_f64 * t26425 * t107805 - 6.0_f64 * t1940 * t95964 * t106565;
    t110792
}
