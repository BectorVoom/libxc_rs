//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1125/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1125<F: Float>(t3668: F, t6856: F, t3217: F, t6555: F, t1851: F, t5336: F, t1262: F, t6774: F, t6837: F, t6496: F, t9545: F, t19904: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t67159 = t6856 * t3668;
    let t67493 = t3217 * t6555;
    let t68040 = t1851 * t5336;
    let t68045 = t6774 * t1262;
    let t68901 = t6837 * t1262;
    let t69078 = t9545 * t6496;
    let t69377 = t19904 * sigma0;
    (t67159, t67493, t68040, t68045, t68901, t69078, t69377)
}
