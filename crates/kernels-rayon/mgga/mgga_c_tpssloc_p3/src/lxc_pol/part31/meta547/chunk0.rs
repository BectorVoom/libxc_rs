//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1772/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1772(t23124: f64, t81902: f64, t23138: f64, t6604: f64, t6606: f64, t22690: f64, t2627: f64, t10024: f64, t1899: f64, t2693: f64, t6609: f64, t213: f64, t6589: f64, t9223: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81903 = t81902 * t23124;
    let t81911 = t23138 * t6604;
    let t81912 = t81911 * t6606;
    let t81914 = t22690 * t2627;
    let t81920 = t1899 * t10024;
    let t81928 = t6609 * t2693;
    let t81933 = t9223 * t6589 * t213;
    (t81903, t81911, t81912, t81914, t81920, t81928, t81933)
}
