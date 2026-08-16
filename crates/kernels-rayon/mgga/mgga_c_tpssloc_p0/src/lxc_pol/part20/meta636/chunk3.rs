//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2340/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2340(t14165: f64, t42841: f64, t10254: f64, t12652: f64, t10241: f64, t10259: f64, t13835: f64, t13839: f64, t13861: f64, t17748: f64, t2986: f64, t2988: f64, t42889: f64, t42893: f64, t42895: f64, t42903: f64, t42906: f64, t43065: f64, t4518: f64, t47701: f64) -> (f64, f64) {
    let t47941 = t42841 * t14165;
    let t47966 = t10254 * t12652;
    let t47978 = 0.27160493827160493826e-2_f64 * t42889 + 0.3086419753086419753e-3_f64 * t42893 - 0.54320987654320987653e-2_f64 * t42895 + 0.16666666666666666666e-2_f64 * t2986 * t10241 * t13835 - 0.11111111111111111111e-2_f64 * t2986 * t43065 * t13839 - 0.83333333333333333331e-3_f64 * t2986 * t10241 * t17748 + 0.33333333333333333333e-2_f64 * t2986 * t2988 * t47966 - 0.83333333333333333331e-3_f64 * t2986 * t10259 * t13861 + 0.49999999999999999999e-2_f64 * t2986 * t4518 * t47701 - 0.9259259259259259259e-3_f64 * t42903 + 0.55555555555555555554e-3_f64 * t42906;
    (t47941, t47978)
}
