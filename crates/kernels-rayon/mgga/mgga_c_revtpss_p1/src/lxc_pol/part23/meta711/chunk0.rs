//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2468/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2468(t47944: f64, t14078: f64, t2470: f64, t3915: f64, t13735: f64, t2435: f64, t10115: f64, t1900: f64, t14189: f64, t22: f64, t46389: f64, t543: f64, t5735: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47945 = 0.39029762157531132076e-1_f64 * t47944;
    let t47947 = t3915 * t14078 * t2470;
    let t47948 = 0.39029762157531132076e-1_f64 * t47947;
    let t47952 = t2435 * t13735;
    let t47953 = 0.21951497276451705329e-1_f64 * t47952;
    let t47961 = t10115 * t1900;
    let t47963 = t2435 * t14189;
    let t47964 = 0.21951497276451705329e-1_f64 * t47963;
    let t47967 = t46389 * t5735 * t543 * t22;
    (t47945, t47948, t47953, t47961, t47964, t47967)
}
