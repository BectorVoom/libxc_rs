//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 879/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk879(t43907: f64, t36506: f64, t959: f64, t11845: f64, t2628: f64, t13625: f64, t2684: f64, t7354: f64, t13626: f64, t2013: f64, t11724: f64, t2464: f64, t2465: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45711 = 0.3575048995185042667e0_f64 * t43907;
    let t45712 = t36506 * t959;
    let t45713 = 0.14896037479937677779e-1_f64 * t45712;
    let t45716 = t11845 * t2628;
    let t45717 = 0.29792074959875355558e-1_f64 * t45716;
    let t45723 = t2684 * t7354 * t13625;
    let t45725 = t2013 * t13626;
    let t45729 = t825 * t2464 * t2465 * t11724;
    (t45711, t45713, t45717, t45723, t45725, t45729)
}
