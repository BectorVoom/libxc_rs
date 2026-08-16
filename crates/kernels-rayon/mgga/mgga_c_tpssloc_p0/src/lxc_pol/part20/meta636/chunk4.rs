//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2341/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2341(t10186: f64, t10255: f64, t13806: f64, t13851: f64, t13871: f64, t2986: f64, t42775: f64, t42964: f64, t42968: f64, t42974: f64, t4510: f64, t4514: f64, t4518: f64, t47684: f64, t47726: f64, t47746: f64, t47767: f64) -> f64 {
    let t48017 = -0.74074074074074074072e-3_f64 * t42964 + 0.98765432098765432095e-3_f64 * t42968 + 0.28806584362139917695e-3_f64 * t42974 - 0.66666666666666666665e-2_f64 * t2986 * t4510 * t47684 - 0.55555555555555555554e-3_f64 * t2986 * t4518 * t47767 + 0.13333333333333333332e-1_f64 * t2986 * t4510 * t47726 + 0.22222222222222222221e-2_f64 * t10186 * t13871 - 0.27777777777777777777e-3_f64 * t2986 * t42775 * t4514 - 0.66666666666666666664e-2_f64 * t2986 * t4518 * t47746 + 0.16666666666666666666e-2_f64 * t2986 * t13851 * t10255 - 0.44444444444444444442e-2_f64 * t10186 * t13806;
    t48017
}
