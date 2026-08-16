//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1101/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1101(t1634: f64, t8396: f64, t10165: f64, t8406: f64, t3174: f64, t1955: f64, t7624: f64, t225: f64, t387: f64, t7593: f64, t345: f64, t1539: f64, t30877: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32908 = t8396 * t1634;
    let t32909 = t10165 * t32908;
    let t32912 = t8406 * t1634;
    let t32913 = t3174 * t32912;
    let t32916 = t1955 * t7624;
    let t32917 = t3174 * t32916;
    let t32923 = t7593 * t225 * t387;
    let t32924 = t345 * t32923;
    let t32927 = t30877 * t1539;
    (t32909, t32913, t32917, t32923, t32924, t32927)
}
