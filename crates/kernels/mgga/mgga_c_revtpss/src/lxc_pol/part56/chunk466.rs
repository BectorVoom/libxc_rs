//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 466/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk466<F: Float>(t235: F, t3999: F, t543: F, t531: F, t549: F, t240: F, t72: F, t1386: F, t2482: F, t27: F, t136: F, t1389: F, t1399: F, t221: F, t1317: F, t1331: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4000 = t3999 * t235;
    let t4003 = t543 * t543;
    let t4010 = 1.0 / t549 / t531;
    let t4011 = t240 * t4010;
    let t4012 = t4011 * t72;
    let t4018 = t2482 * t1386 * t27;
    let t4019 = t1389 * t136;
    let t4021 = t4019 * t221 * t1399;
    let t4022 = t4018 * t4021;
    let t4024 = t1317 * t1331;
    (t4000, t4003, t4010, t4012, t4018, t4019, t4021, t4022, t4024)
}
