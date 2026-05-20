//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1604/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1604<F: Float>(t17353: F, t17514: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t1214: F, t4186: F, t5296: F, t1042: F, t1469: F, t3584: F) -> (F, F, F, F, F) {
    let t17515 = t17353 * t17514;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17534 = t4186 * t1214;
    let t17535 = t5296 * t17534;
    let t17536 = t1042 * t17535;
    let t17539 = t1469 * t3584;
    (t17515, t17525, t17529, t17536, t17539)
}
