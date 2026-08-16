//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1318/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1318(t10686: f64, t107: f64, t10955: f64, t10958: f64, t11020: f64, t2021: f64, t2023: f64, t2194: f64, t2197: f64, t28726: f64, t28729: f64, t28731: f64, t33584: f64, t33586: f64, t33590: f64, t33604: f64, t33607: f64, t33610: f64, t33613: f64, t33616: f64, t33619: f64, t6159: f64) -> f64 {
    let t33620 = -t33584 + t33586 - t28726 + t28729 + t28731 - t33590 + 0.79445533226334281486e-1_f64 * t2021 * t10686 * t107 * t2023 - 0.46011511144704899612e1_f64 * t6159 * t11020 - 0.92023022289409799224e1_f64 * t2194 * t10955 + 0.23005755572352449806e2_f64 * t2197 * t10958 - t33604 + t33607 + t33610 - t33613 + t33616 - t33619;
    t33620
}
