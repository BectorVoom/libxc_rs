//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1022/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1022(t13242: f64, t4180: f64, t4182: f64, t4181: f64, t9632: f64, t2642: f64, t4166: f64, t2617: f64, t4177: f64, t2628: f64, t836: f64, t812: f64) -> (f64, f64, f64, f64, f64) {
    let t13244 = t4180 * t13242 * t4182;
    let t13248 = t4180 * t4181 * t9632;
    let t13251 = t4166 * t2642;
    let t13254 = t2617 * t4177;
    let t13257 = t2628 * t836;
    let t13258 = t812 * t13257;
    (t13244, t13248, t13251, t13254, t13258)
}
