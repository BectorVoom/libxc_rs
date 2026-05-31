//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 548/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk548<F: Float>(t1123: F, t2861: F, t984: F, t987: F, t983: F, t990: F, t110: F, t292: F, t285: F, t24: F, t992: F) -> (F, F, F, F, F, F) {
    let t2862 = t2861 * t1123;
    let t2870 = t984 * t987;
    let t2872 = t983 * t990;
    let t2877 = t110 * t292;
    let t2879 = t285 * t2877 / F::cast_from(432.0_f64);
    let t2880 = t24 * t992;
    (t2862, t2870, t2872, t2877, t2879, t2880)
}
