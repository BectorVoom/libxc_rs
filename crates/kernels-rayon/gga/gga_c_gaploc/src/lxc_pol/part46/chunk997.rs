//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 997/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk997(t41295: f64, t41299: f64, t41305: f64, t41307: f64, t41312: f64, t41316: f64, t13016: f64, t8478: f64, t13097: f64, t1445: f64, t2009: f64, t43270: f64, t43861: f64, t43864: f64, t43870: f64, t43875: f64, t43879: f64, t43882: f64, t43883: f64, t43884: f64, t43885: f64, t43886: f64, t43887: f64, t773: f64, t813: f64) -> f64 {
    let t43888 = 0.63904876589867916127e-1_f64 * t41295;
    let t43889 = 0.63904876589867916127e-1_f64 * t41299;
    let t43890 = 0.59584149919750711116e-1_f64 * t41305;
    let t43891 = 0.89376224879626066674e-1_f64 * t41307;
    let t43892 = 0.63904876589867916127e-1_f64 * t41312;
    let t43893 = 0.63904876589867916127e-1_f64 * t41316;
    let t43895 = 0.10725146985555128001e1_f64 * t8478 * t13016;
    let t43899 = t43861 + t43864 - 0.46011511144704899612e1_f64 * t813 * t1445 * t43270 - 0.13803453343411469884e2_f64 * t43870 + t43875 - t43879 + t43882 + t43883 + t43884 - t43885 - t43886 + t43887 - t43888 - t43889 + t43890 - t43891 + t43892 + t43893 - t43895 - 0.35750489951850426669e0_f64 * t773 * t13097 * t2009;
    t43899
}
