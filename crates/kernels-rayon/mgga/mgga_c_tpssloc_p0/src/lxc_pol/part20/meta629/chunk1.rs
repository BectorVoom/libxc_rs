//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2283/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2283(t47172: f64, t708: f64, t41295: f64, t157: f64, t41284: f64, t12940: f64, t12923: f64, t12939: f64, t2244: f64, t12892: f64, t12908: f64, t2250: f64, t4194: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47174 = 12.0_f64 * t47172 * t708;
    let t47175 = 36.0_f64 * t41295;
    let t47176 = t41284 * t157;
    let t47178 = 72.0_f64 * t47176 * t12940;
    let t47180 = t12939 * t12923 * t2244;
    let t47181 = 72.0_f64 * t47180;
    let t47183 = 36.0_f64 * t12908 * t12892;
    let t47185 = t4194 * t12923 * t2250;
    (t47174, t47175, t47178, t47181, t47183, t47185)
}
