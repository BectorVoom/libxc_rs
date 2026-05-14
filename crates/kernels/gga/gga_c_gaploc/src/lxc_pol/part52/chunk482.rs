//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 482/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk482<F: Float>(t10012: F, t723: F, t9438: F, t2684: F, t10007: F, t701: F, t2615: F, t2628: F, t2673: F, t7442: F, t787: F, t2563: F, t900: F, t3295: F, t7354: F, t2365: F, t7069: F) -> (F, F, F, F, F, F, F) {
    let t10013 = t10012 * t723;
    let t10014 = t9438 * t10013;
    let t10015 = t2684 * t10014;
    let t10017 = t10007 * t701;
    let t10018 = t9438 * t10017;
    let t10019 = t2615 * t10018;
    let t10022 = 0.59584149919750711116e-1 * t2673 * t2628;
    let t10023 = t787 * t7442;
    let t10024 = t900 * t2563;
    let t10026 = 0.89376224879626066674e-1 * t10023 * t10024;
    let t10029 = t7354 * t3295;
    let t10030 = t2684 * t10029;
    let t10040 = t2365 * t7069;
    (t10015, t10019, t10022, t10024, t10026, t10030, t10040)
}
