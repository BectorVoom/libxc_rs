//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1723;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta418(t22479: f64, t510: f64, t652: f64, t1976: f64, t2363: f64, t2303: f64, t71: f64, t1863: f64, t33: f64, t9228: f64, t43: f64, t614: f64, t2267: f64, t38: f64, t240: f64, t2244: f64, t2250: f64, t2261: f64, t44: f64, t607: f64, t6500: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22480, t22482, t22483, t22489, t22490, t22493, t22502) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1723(t22479, t510, t652, t1976, t2363, t2303, t71, t1863, t33, t9228, t43, t614);
        let (t22505, t22510, t22511, t22512, t22513) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1724(t2267, t38, t240, t2244, t2250, t22502, t2261, t44, t607, t6500, t67, t1864);
    (t22480, t22482, t22483, t22489, t22490, t22493, t22502, t22505, t22510, t22511, t22512, t22513)
}
