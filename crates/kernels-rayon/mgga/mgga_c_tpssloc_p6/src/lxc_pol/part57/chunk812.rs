//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 812/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk812(t2031: f64, t27956: f64, t1860: f64, t2032: f64, t23963: f64, t23995: f64, t26016: f64, t26911: f64, t26920: f64, t26936: f64, t26948: f64, t26954: f64, t26960: f64, t27937: f64, t27961: f64, t27966: f64, t27972: f64, t27976: f64, t27979: f64, t27982: f64, t7026: f64, t7428: f64, t7432: f64, t7435: f64, t7782: f64) -> f64 {
    let t28935 = t2031 * t27956;
    let t28941 = t27937 * t2032 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7428 * t7782 + 10.0_f64 * t23963 * t27961 + 20.0_f64 / 3.0_f64 * t26016 * t26954 + t23995 - 10.0_f64 / 3.0_f64 * t7026 * t27972 - 5.0_f64 / 3.0_f64 * t7026 * t27976 - 2.0_f64 / 3.0_f64 * t27979 * t2032 - 2.0_f64 / 3.0_f64 * t27982 * t2032 - 4.0_f64 / 3.0_f64 * t7435 * t7782 - 16.0_f64 / 9.0_f64 * t26948 - 10.0_f64 / 3.0_f64 * t26911 * t7432 - 4.0_f64 / 3.0_f64 * t27966 * t2032 + t1860 * t28935 / 3.0_f64 + 80.0_f64 / 9.0_f64 * t26920 - 16.0_f64 / 9.0_f64 * t26960 + 32.0_f64 / 9.0_f64 * t26936;
    t28941
}
