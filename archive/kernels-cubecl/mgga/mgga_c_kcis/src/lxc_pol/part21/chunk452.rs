//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 452/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk452<F: Float>(t2865: F, t69: F, t984: F, t987: F, t983: F, t990: F, sigma0: F) -> (F, F, F, F) {
    let t2866 = sigma0 * t2865;
    let t2867 = t2866 * t69;
    let t2870 = t984 * t987;
    let t2872 = t983 * t990;
    (t2866, t2867, t2870, t2872)
}
