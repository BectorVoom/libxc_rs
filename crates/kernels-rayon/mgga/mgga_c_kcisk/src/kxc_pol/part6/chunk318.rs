//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 318/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk318(t1646: f64, t677: f64, t1634: f64, t1806: f64, t429: f64, t446: f64, t686: f64, t41: f64, t569: f64) -> (f64, f64, f64, f64, f64) {
    let t1815 = t1646 * t677;
    let t1819 = 0.41275e-2_f64 * t1634;
    let t1829 = 0.11955719325063177623e-1_f64 * t1806;
    let t1834 = 0.3513e-2_f64 * t429 * t446 * t686;
    let t1835 = t41 * t569;
    (t1815, t1819, t1829, t1834, t1835)
}
