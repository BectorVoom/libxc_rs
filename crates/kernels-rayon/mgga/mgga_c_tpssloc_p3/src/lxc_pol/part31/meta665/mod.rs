//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta665(t16944: f64, t25891: f64, t25927: f64, t98111: f64, t1649: f64, t4119: f64, t23788: f64, t67123: f64, t1081: f64, t5660: f64, t5544: f64, t16662: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t100708, t100713, t100718, t100731, t100734, t100743, t100747) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1954(t16944, t25891, t25927, t98111, t1649, t4119, t23788, t67123, t1081, t5660, t5544, t16662, t28);
    (t100708, t100713, t100718, t100731, t100734, t100743, t100747)
}
