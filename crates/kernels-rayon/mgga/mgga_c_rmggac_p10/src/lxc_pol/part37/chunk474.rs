//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 474/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk474(t291: f64, t7755: f64, t13823: f64, t2123: f64, t649: f64, t27: f64, t2145: f64, t3076: f64, t321: f64, t2044: f64, t12200: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13824 = t7755 * t291;
    let t13825 = t13823 * t13824;
    let t13827 = t649 * t2123;
    let t13828 = t27 * t13827;
    let t13829 = t2145 * t13828;
    let t13831 = t3076 * t321;
    let t13832 = t2044 * t13831;
    let t13833 = t12200 * t13832;
    let t13835 = t3076 * t333;
    (t13824, t13825, t13828, t13829, t13832, t13833, t13835)
}
