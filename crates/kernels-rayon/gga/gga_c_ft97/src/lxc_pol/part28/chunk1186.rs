//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1186/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1186(t1017: f64, t1023: f64, t1349: f64, t1360: f64, t147894: f64, t148196: f64, t148686: f64, t148703: f64, t148722: f64, t148726: f64, t148955: f64, t160: f64, t26793: f64, t27406: f64, t28: f64, t3313: f64, t33221: f64, t3414: f64, t5778: f64, t5973: f64, t7309: f64, t7412: f64) -> f64 {
    let t149630 = -2.0_f64 * t147894 + 2.0_f64 * t148686 * t160 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t5778 * t5973 * t1017 - t7309 * t26793 / 3.0_f64 - 4.0_f64 * t148726 - t3313 * t7412 - 2.0_f64 * t148722 - t1023 * t33221 - t3414 * t7412 - 4.0_f64 * t148703 - 4.0_f64 * t148196 + 4.0_f64 * t148955 + t1349 * t28 * t1360 * t27406 / 3.0_f64;
    t149630
}
