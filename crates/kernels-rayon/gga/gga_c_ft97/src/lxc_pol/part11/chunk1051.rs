//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1051/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1051(t41861: f64, t446: f64, t9744: f64, t713: f64, t9587: f64, t2354: f64, t41482: f64, t724: f64, t2594: f64, t41464: f64, t41823: f64, t41829: f64, t41831: f64, t41835: f64, t41839: f64, t41844: f64, t41846: f64, t41852: f64, t41856: f64, t41859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41863 = t446 * t9744 * t41861;
    let t41865 = t9587 * t713;
    let t41867 = t446 * t2354 * t41865;
    let t41870 = t446 * t724 * t41482;
    let t41873 = t446 * t2594 * t41464;
    let t41875 = -10.0_f64 / 27.0_f64 * t41823 + 4.0_f64 / 3.0_f64 * t41829 + 4.0_f64 / 9.0_f64 * t41831 - 2.0_f64 / 3.0_f64 * t41835 - 2.0_f64 / 3.0_f64 * t41839 + t41844 + 4.0_f64 / 3.0_f64 * t41846 + 4.0_f64 * t41852 + t41856 + t41859 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t41863 + 4.0_f64 / 3.0_f64 * t41867 - 2.0_f64 * t41870 + 4.0_f64 / 3.0_f64 * t41873;
    (t41863, t41865, t41867, t41870, t41873, t41875)
}
