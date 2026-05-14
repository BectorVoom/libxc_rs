//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1381/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1381<F: Float>(t31572: F, t44280: F, t446: F, t824: F, t25044: F, t2665: F, t4635: F, t126368: F, t24976: F, t6317: F, t125858: F, t125862: F, t125826: F, t99559: F, t43381: F, t10248: F, t125971: F) -> (F, F, F, F, F, F, F, F) {
    let t127763 = t446 * t44280 * t31572 * t824;
    let t127767 = t446 * t2665 * t25044 * t4635;
    let t127770 = t6317 * t24976 * t126368;
    let t127773 = t6317 * t24976 * t125858;
    let t127776 = t6317 * t24976 * t125862;
    let t127779 = t6317 * t99559 * t125826;
    let t127781 = t446 * t43381 * t125826;
    let t127784 = t446 * t10248 * t125971;
    (t127763, t127767, t127770, t127773, t127776, t127779, t127781, t127784)
}
