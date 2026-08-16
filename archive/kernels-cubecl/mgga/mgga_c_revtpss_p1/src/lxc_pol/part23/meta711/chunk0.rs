//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2468/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2468<F: Float>(t47944: F, t14078: F, t2470: F, t3915: F, t13735: F, t2435: F, t10115: F, t1900: F, t14189: F, t22: F, t46389: F, t543: F, t5735: F) -> (F, F, F, F, F, F) {
    let t47945 = F::cast_from(0.39029762157531132076e-1_f64) * t47944;
    let t47947 = t3915 * t14078 * t2470;
    let t47948 = F::cast_from(0.39029762157531132076e-1_f64) * t47947;
    let t47952 = t2435 * t13735;
    let t47953 = F::cast_from(0.21951497276451705329e-1_f64) * t47952;
    let t47961 = t10115 * t1900;
    let t47963 = t2435 * t14189;
    let t47964 = F::cast_from(0.21951497276451705329e-1_f64) * t47963;
    let t47967 = t46389 * t5735 * t543 * t22;
    (t47945, t47948, t47953, t47961, t47964, t47967)
}
