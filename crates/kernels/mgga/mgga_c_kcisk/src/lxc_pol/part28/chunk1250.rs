//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1250/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1250<F: Float>(t9990: F, t9999: F, t24608: F, t79: F, t2803: F, t779: F, t8831: F, t20: F, t2801: F, t2029: F, t9176: F, t5508: F, t1586: F, t2063: F, t2647: F, t33226: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35427 = t9990 * t9999;
    let t35430 = t24608 * t79;
    let t35431 = t35430 * t2803;
    let t35437 = t779 * t8831;
    let t35438 = t35437 * t20;
    let t35439 = t2801 * t35438;
    let t35444 = t2029 * t9176;
    let t35445 = t5508 * t35444;
    let t35446 = t1586 * t35445;
    let t35453 = t33226 * t2063 * t2647;
    (t35427, t35430, t35431, t35437, t35438, t35439, t35444, t35445, t35446, t35453)
}
