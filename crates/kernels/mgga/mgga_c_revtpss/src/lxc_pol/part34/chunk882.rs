//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 882/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk882<F: Float>(t22767: F, t23063: F, t23077: F, t23092: F, t14312: F, t18301: F, t1522: F, t18263: F, t14328: F, t14334: F, t10552: F, t10554: F, t2403: F, t4546: F, t5962: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F) {
    let t23094 = t22767 + t23063 + t23077 + t23092;
    let t23096 = 3.0 * t14312;
    let t23097 = 3.0 * t18301;
    let t23102 = 12.0 * t18263 * t1522;
    let t23103 = 0.35089341735807877242e1 * t14328;
    let t23104 = 0.17544670867903938621e1 * t14334;
    let t23105 = 9.0 * t2403 * t4546 * t5962 - t10552 + t10554 + t23096 + t23097 + t23102 + t23103 - t23104 - t9278 + t9308 + t9316 + t9329 + t9333;
    (t23094, t23096, t23097, t23102, t23103, t23104, t23105)
}
