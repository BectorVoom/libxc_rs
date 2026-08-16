//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2310/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2310<F: Float>(t23384: F, t28657: F, t1058: F, t1060: F, t1615: F, t1625: F, t18107: F, t23327: F, t23346: F, t23613: F, t23633: F, t25429: F, t25510: F, t25549: F, t25705: F, t25713: F, t2770: F, t2775: F, t28609: F, t28614: F, t3200: F, t3961: F, t6687: F, t6743: F, t7619: F, t883: F, t89309: F, t89310: F, t89327: F, t99180: F) -> F {
    let t100254 = t23384 * t28657;
    let t100287 = -F::cast_from(0.27415567780803773942e-2_f64) * t100254 - F::cast_from(0.73108180748810063845e-2_f64) * t23346 * t28614 - t89309 - F::cast_from(0.36554090374405031923e-2_f64) * t89310 - F::cast_from(2.0_f64) * t3200 * t7619 * t18107 + t89327 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t25510 * t1625 * t2775 * t3961 + F::cast_from(0.73108180748810063845e-2_f64) * t25429 * t25510 * t1625 * t2770 * t3961 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23613 * t28609 + F::cast_from(2.0_f64) * t1058 * t25705 * t1615 * t1060 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t99180 * t25713 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t6743 * t1625 * t883 * t25549;
    t100287
}
