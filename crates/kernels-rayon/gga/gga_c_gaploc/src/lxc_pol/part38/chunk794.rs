//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 794/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk794(t2748: f64, t3113: f64, t12964: f64, t2487: f64, t6985: f64, t10615: f64, t1423: f64, t3129: f64, t40377: f64, t2890: f64, t9267: f64, t9278: f64) -> (f64, f64, f64, f64, f64) {
    let t42115 = t2748 * t3113;
    let t42146 = t2487 * t6985 * t12964;
    let t42156 = t10615 * t1423 * t3129;
    let t42170 = 0.19171462976960374838e0_f64 * t40377;
    let t42183 = t9267 * t2890 * t9278;
    (t42115, t42146, t42156, t42170, t42183)
}
