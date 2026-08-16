//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 379/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk379(t1757: f64, t1899: f64, t1800: f64, t1869: f64, t1689: f64, t1693: f64, t1792: f64, t1796: f64, t1804: f64, t1866: f64, t1897: f64, t671: f64) -> (f64, f64, f64, f64) {
    let t1900 = t1899 * t1757;
    let t1901 = t1800 * t1900;
    let t1902 = t1869 * t1901;
    let t1904 = t1689 * t671 - 0.193e0_f64 * t1693 * t1792 + t1796 + 0.16581944444444444444e-2_f64 * t1804 + 0.24872916666666666666e-2_f64 * t1866 - 0.24872916666666666666e-2_f64 * t1897 + 0.16581944444444444444e-2_f64 * t1902;
    (t1900, t1901, t1902, t1904)
}
