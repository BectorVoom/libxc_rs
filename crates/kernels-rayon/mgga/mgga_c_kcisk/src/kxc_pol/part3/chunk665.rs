//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 665/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk665(t10536: f64, t1869: f64, t4736: f64, t4744: f64, t1663: f64, t4742: f64, t45: f64, t4753: f64, t4781: f64, t4787: f64, t6880: f64, t1683: f64, t4762: f64) -> (f64, f64, f64, f64, f64) {
    let t10537 = t1869 * t10536;
    let t10539 = t4736 * t4744;
    let t10540 = t10539 * t1663;
    let t10542 = 0.48245472966453314466e2_f64 * t4742 * t10540;
    let t10543 = t45 * t4753;
    let t10549 = t4787 * t4781 * t6880;
    let t10552 = t4762 * t1683;
    (t10537, t10542, t10543, t10549, t10552)
}
