//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2954/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2954(t13798: f64, t17863: f64, t2979: f64, t2980: f64, t2986: f64, t43065: f64, t4514: f64, t48180: f64, t48191: f64, t48373: f64, t48378: f64, t48381: f64, t48384: f64, t48387: f64, t48390: f64, t48394: f64, t55723: f64, t59706: f64, t59711: f64, t973: f64) -> f64 {
    let t61585 = 0.49382716049382716048e-3_f64 * t48373 - 0.32921810699588477366e-3_f64 * t48378 + 0.24691358024691358024e-3_f64 * t48381 - 0.74074074074074074072e-3_f64 * t48384 - 0.37037037037037037036e-3_f64 * t48387 - 0.14814814814814814814e-2_f64 * t48390 + 0.49382716049382716048e-3_f64 * t48394 + 0.74074074074074074072e-3_f64 * t973 * t2979 * t2980 * t55723 + 0.86419753086419753084e-3_f64 * t2986 * t13798 * t59706 + 0.28806584362139917695e-2_f64 * t2986 * t48180 * t59711 - 0.37037037037037037036e-3_f64 * t2986 * t43065 * t17863 - 0.55555555555555555554e-3_f64 * t2986 * t48191 * t4514;
    t61585
}
