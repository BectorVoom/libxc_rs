//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1110/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1110<F: Float>(t33017: F, t5187: F, t1799: F, t1791: F, t5030: F, t5032: F, t7261: F, t1894: F, t642: F, t1757: F, t1869: F, t18325: F, t9663: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33018 = t33017 * t5187;
    let t33019 = t1799 * t33018;
    let t33021 = t5030 * t1791;
    let t33022 = t33021 * t5032;
    let t33023 = t7261 * t33022;
    let t33026 = t642 * t1894;
    let t33027 = t33026 * t1757;
    let t33028 = t33017 * t33027;
    let t33029 = t1869 * t33028;
    let t33031 = t9663 * t18325;
    (t33018, t33019, t33021, t33022, t33023, t33027, t33028, t33029, t33031)
}
