//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 798/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk798<F: Float>(t5713: F, t617: F, t12844: F, t6172: F, t4439: F, t1607: F, t110: F, t2105: F, t1599: F, t25: F, t6184: F, t4429: F, t6141: F, t12825: F, t2099: F, t6155: F) -> (F, F, F, F, F, F, F, F) {
    let t18079 = t5713 * t617;
    let t18091 = t12844 * t6172;
    let t18093 = t4439 * t18091 / 864.0;
    let t18128 = t5713 * t1607;
    let t18141 = t110 * t2105;
    let t18142 = t1599 * t18141;
    let t18146 = t25 * t6184;
    let t18148 = t1599 * t18146 / 288.0;
    let t18152 = t6141 * t4429 / 108.0;
    let t18163 = t12825 * t2099;
    let t18164 = t1599 * t18163;
    let t18168 = t12844 * t6155;
    (t18079, t18093, t18128, t18142, t18148, t18152, t18164, t18168)
}
