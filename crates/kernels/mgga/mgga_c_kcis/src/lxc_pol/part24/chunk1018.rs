//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1018/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1018<F: Float>(t1262: F, t6774: F, t6837: F, t6496: F, t9545: F, t19904: F, t1130: F, t6613: F, t6486: F, t3643: F, t6835: F, t1239: F, t20550: F, t1281: F, t20709: F, t31297: F, t6301: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t68045 = t6774 * t1262;
    let t68901 = t6837 * t1262;
    let t69078 = t9545 * t6496;
    let t69377 = t19904 * sigma0;
    let t69560 = t1130 * t6613;
    let t70032 = t1130 * t6486;
    let t70071 = t6835 * t3643;
    let t70078 = t20550 * t1239;
    let t70451 = t20709 * t1281;
    let t70767 = t6301 * t31297;
    (t68045, t68901, t69078, t69377, t69560, t70032, t70071, t70078, t70451, t70767)
}
