//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 739/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk739(t20868: f64, t9133: f64, t12752: f64, t16986: f64, t17060: f64, t17091: f64, t1901: f64, t20744: f64, t20750: f64, t20755: f64, t20760: f64, t20765: f64, t20769: f64, t20853: f64, t20859: f64, t20862: f64, t20865: f64, t28: f64, t89: f64) -> (f64, f64) {
    let t20869 = t9133 * t20868;
    let t20872 = -2.0_f64 / 9.0_f64 * t16986 - 2.0_f64 / 3.0_f64 * t1901 * t20744 + t17060 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t20750 + 2.0_f64 / 9.0_f64 * t1901 * t20755 + t1901 * t20760 / 3.0_f64 + t1901 * t20765 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1901 * t20769 - 2.0_f64 / 3.0_f64 * t17091 + t89 * t28 * t20853 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t12752 - 2.0_f64 / 9.0_f64 * t1901 * t20859 + 2.0_f64 / 3.0_f64 * t1901 * t20862 + 2.0_f64 / 3.0_f64 * t1901 * t20865 - 2.0_f64 / 3.0_f64 * t1901 * t20869;
    (t20869, t20872)
}
