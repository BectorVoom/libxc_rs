//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 522/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk522(t1413: f64, t2257: f64, t1556: f64, t2306: f64, t260: f64, t338: f64, t67: f64, t41: f64, t4143: f64, t1576: f64, t2318: f64, t2317: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6387 = t2257 * t1413;
    let t6388 = t6387 * sigma0;
    let t6426 = t2306 * t1556;
    let t6442 = t260 * t67 * t338;
    let t6443 = t41 * t4143;
    let t6456 = t2318 * t1576;
    let t6458 = t2317 * sigma0;
    (t6387, t6388, t6426, t6442, t6443, t6456, t6458)
}
