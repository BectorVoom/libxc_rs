//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 595/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk595(t1312: f64, t8331: f64, t2326: f64, t4375: f64, t1586: f64, t4423: f64, t5668: f64, t7738: f64, t7742: f64, t7746: f64, t2292: f64, t1537: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8332 = t1312 * t8331;
    let t8335 = t2326 * t2326;
    let t8336 = t4375 * t8335;
    let t8337 = t1586 * t8336;
    let t8344 = t4423 + 0.11415555555555555555e-1_f64 * t5668 - 0.11415555555555555555e-1_f64 * t7738 + 0.34246666666666666666e-1_f64 * t7742 - 0.17123333333333333333e-1_f64 * t7746;
    let t8349 = t2292 * t2292;
    let t8350 = t8349 * t1537;
    (t8332, t8335, t8336, t8337, t8344, t8349, t8350)
}
