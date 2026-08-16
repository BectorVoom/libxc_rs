//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 689/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk689(t13966: f64, t2046: f64, t7385: f64, t14082: f64, t14089: f64, t4789: f64, t68949: f64, t3046: f64, t880: f64, t899: f64, t2144: f64, t1550: f64, t7778: f64, t7799: f64) -> (f64, f64, f64, f64, f64) {
    let t69016 = t2046 * t13966 * t7385;
    let t69027 = t14089 * t14082 * t4789 * t68949;
    let t69041 = t899 * t880 * t3046;
    let t69045 = t899 * t2144 * t3046;
    let t69049 = t1550 * t7778 * t7799;
    (t69016, t69027, t69041, t69045, t69049)
}
