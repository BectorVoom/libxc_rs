//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 998/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk998<F: Float>(t41295: F, t41299: F, t41305: F, t41307: F, t41312: F, t41316: F, t13016: F, t8478: F, t13097: F, t1445: F, t2009: F, t43270: F, t43861: F, t43864: F, t43870: F, t43875: F, t43879: F, t43882: F, t43883: F, t43884: F, t43885: F, t43886: F, t43887: F, t773: F, t813: F) -> F {
    let t43888 = F::cast_from(0.63904876589867916127e-1_f64) * t41295;
    let t43889 = F::cast_from(0.63904876589867916127e-1_f64) * t41299;
    let t43890 = F::cast_from(0.59584149919750711116e-1_f64) * t41305;
    let t43891 = F::cast_from(0.89376224879626066674e-1_f64) * t41307;
    let t43892 = F::cast_from(0.63904876589867916127e-1_f64) * t41312;
    let t43893 = F::cast_from(0.63904876589867916127e-1_f64) * t41316;
    let t43895 = F::cast_from(0.10725146985555128001e1_f64) * t8478 * t13016;
    let t43899 = t43861 + t43864 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t43270 - F::cast_from(0.13803453343411469884e2_f64) * t43870 + t43875 - t43879 + t43882 + t43883 + t43884 - t43885 - t43886 + t43887 - t43888 - t43889 + t43890 - t43891 + t43892 + t43893 - t43895 - F::cast_from(0.35750489951850426669e0_f64) * t773 * t13097 * t2009;
    t43899
}
