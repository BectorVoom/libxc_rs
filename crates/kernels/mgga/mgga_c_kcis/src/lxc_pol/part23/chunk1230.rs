//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1230/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1230<F: Float>(t1394: F, t27379: F, t28356: F, t27383: F, t4153: F, t28361: F, t3728: F, t94223: F, t94225: F, t98030: F, t98036: F, t98039: F, t98043: F, t98046: F, t98049: F) -> (F, F, F, F) {
    let t98052 = t1394 * t28356 * t27379;
    let t98055 = t4153 * t28356 * t27383;
    let t98057 = t3728 * t28361;
    let t98058 = F::new(0.22109259259259259258e-2) * t98057;
    let t98059 = -F::new(0.16581944444444444444e-2) * t98030 - F::new(0.22109259259259259258e-2) * t94223 + F::new(0.14739506172839506172e-2) * t94225 + F::new(0.3684876543209876543e-2) * t98036 - F::new(0.22109259259259259258e-2) * t98039 + F::new(0.99491666666666666664e-2) * t98043 + F::new(0.16581944444444444444e-2) * t98046 + F::new(0.27636574074074074073e-2) * t98049 - F::new(0.44218518518518518517e-2) * t98052 - F::new(0.73697530864197530862e-2) * t98055 - t98058;
    (t98052, t98055, t98057, t98059)
}
