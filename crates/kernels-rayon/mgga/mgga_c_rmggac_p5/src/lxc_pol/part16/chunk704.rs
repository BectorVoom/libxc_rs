//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 704/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk704(t10166: f64, t262: f64, t7835: f64, t7844: f64, t9885: f64, t4669: f64, t9712: f64, t7782: f64, t9713: f64, t7785: f64, t9709: f64, t7788: f64, t9705: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10168 = t7835 * t262 * t10166;
    let t10170 = t7844 * t9885;
    let t10177 = t4669 * t9712;
    let t10179 = t7782 * t9713;
    let t10181 = t7785 * t9709;
    let t10183 = t7788 * t9705;
    (t10168, t10170, t10177, t10179, t10181, t10183)
}
