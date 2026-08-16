//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1851;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta519(t3953: f64, t608: f64, t1437: f64, t641: f64, t72: f64, t4021: f64, t79: f64, t1410: f64, t2235: f64, t3961: f64, t605: f64, t3967: f64, t33: f64, t7440: f64, t2240: f64, t1433: f64, t645: f64, t1865: f64, t22523: f64, t22554: f64, t6490: f64, t6492: f64, t6495: f64, t6506: f64, t6510: f64, t7432: f64, t7435: f64, t7442: f64, t7446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26055, t26063, t26067, t26070, t26073, t26076) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1851(t3953, t608, t1437, t641, t72, t4021, t79, t1410, t2235, t3961, t605, t3967);
        let (t26083, t26084, t26090, t26095) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1852(t33, t7440, t2240, t1433, t645, t72, t1865, t22523, t22554, t26055, t26063, t26067, t26070, t26073, t26076, t6490, t6492, t6495, t6506, t6510, t7432, t7435, t7442, t7446);
    (t26055, t26063, t26067, t26070, t26073, t26076, t26083, t26084, t26090, t26095)
}
