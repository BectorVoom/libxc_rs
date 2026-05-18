//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 638/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk638<F: Float>(t1282: F, t187: F, t1872: F, t3669: F, t437: F, t5360: F, t6635: F, t6637: F, t6640: F, t6736: F, t6856: F, t6860: F, t6879: F) -> F {
    let t6883 = t6635 - t6637 + t6640 - t6736 + t187 * (-t1282 * t6879 - F::new(2.0) * t1872 * t5360 + F::new(2.0) * t3669 * t6860 + t437 * t6856 - t6635 + t6637 - t6640 + t6736);
    t6883
}
