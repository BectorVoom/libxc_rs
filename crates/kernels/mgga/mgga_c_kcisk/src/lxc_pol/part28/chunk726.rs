//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 726/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk726<F: Float>(t716: F, t9014: F, t736: F, t1755: F, t8780: F, t735: F, t734: F, t2580: F, t7320: F, t2560: F, t2568: F, t2572: F, t719: F, t8939: F, t1935: F, t2564: F, t7337: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9015 = t9014 * t716;
    let t9016 = t9015 * sigma2;
    let t9017 = t9016 * t736;
    let t9019 = t1755 * t8780;
    let t9020 = t735 * t9019;
    let t9021 = t734 * t9020;
    let t9023 = t7320 * t2580;
    let t9025 = t2560 * t2568;
    let t9027 = t2560 * t2572;
    let t9029 = t719 * t8939;
    let t9030 = t735 * t9029;
    let t9031 = t1935 * t9030;
    let t9033 = t7337 * t2564;
    (t9015, t9016, t9017, t9019, t9020, t9021, t9023, t9025, t9027, t9029, t9030, t9031, t9033)
}
