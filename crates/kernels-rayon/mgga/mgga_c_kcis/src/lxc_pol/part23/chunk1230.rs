//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1230/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1230(t1394: f64, t27379: f64, t28356: f64, t27383: f64, t4153: f64, t28361: f64, t3728: f64, t94223: f64, t94225: f64, t98030: f64, t98036: f64, t98039: f64, t98043: f64, t98046: f64, t98049: f64) -> (f64, f64, f64, f64) {
    let t98052 = t1394 * t28356 * t27379;
    let t98055 = t4153 * t28356 * t27383;
    let t98057 = t3728 * t28361;
    let t98058 = 0.22109259259259259258e-2_f64 * t98057;
    let t98059 = -0.16581944444444444444e-2_f64 * t98030 - 0.22109259259259259258e-2_f64 * t94223 + 0.14739506172839506172e-2_f64 * t94225 + 0.3684876543209876543e-2_f64 * t98036 - 0.22109259259259259258e-2_f64 * t98039 + 0.99491666666666666664e-2_f64 * t98043 + 0.16581944444444444444e-2_f64 * t98046 + 0.27636574074074074073e-2_f64 * t98049 - 0.44218518518518518517e-2_f64 * t98052 - 0.73697530864197530862e-2_f64 * t98055 - t98058;
    (t98052, t98055, t98057, t98059)
}
