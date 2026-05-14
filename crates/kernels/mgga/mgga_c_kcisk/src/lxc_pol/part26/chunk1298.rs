//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1298/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1298<F: Float>(t115645: F, t9536: F, t1310: F, t4375: F, t114796: F, t5579: F, t9406: F, t33332: F, t33334: F, t33336: F, t33339: F, t33343: F, t33982: F, t33984: F, t33988: F, t33991: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t115950 = 0.11574074074074074074e-2 * t9536 * t115645;
    let t115955 = t1310 * t4375;
    let t115969 = 0.15476481481481481481e-2 * t114796;
    let t116014 = 2.0 * t5579 * t9406;
    let t116027 = t33332 / 8.0;
    let t116028 = t33334 / 8.0;
    let t116029 = t33336 / 8.0;
    let t116031 = t33339 / 8.0;
    let t116032 = t33343 / 8.0;
    let t116033 = t33982 / 8.0;
    let t116036 = t33984 / 8.0;
    let t116037 = t33988 / 8.0;
    let t116038 = t33991 / 8.0;
    (t115950, t115955, t115969, t116014, t116027, t116028, t116029, t116031, t116032, t116033, t116036, t116037, t116038)
}
