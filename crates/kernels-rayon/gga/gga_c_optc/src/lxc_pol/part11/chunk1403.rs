//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1403/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1403(t17423: f64, t4144: f64, t33724: f64, t33730: f64, t43414: f64, t44193: f64, t44198: f64, t52389: f64, t52391: f64, t52393: f64, t58348: f64, t58352: f64, t58356: f64, t58360: f64, t58363: f64, t58367: f64) -> (f64, f64) {
    let t59088 = 4.0_f64 * t4144 * t17423;
    let t59103 = -0.18257037037037037037e0_f64 * t44193 + 0.10954222222222222222e1_f64 * t44198 - 0.5314962962962962963e0_f64 * t43414 + 0.12401580246913580247e1_f64 * t33724 + 0.97370864197530864199e0_f64 * t33730 + 0.23917333333333333333e1_f64 * t58348 + 0.98587999999999999999e0_f64 * t58352 - 0.21908444444444444444e0_f64 * t58356 - 0.295764e1_f64 * t58360 + 0.65725333333333333332e0_f64 * t58363 - 0.10954222222222222222e0_f64 * t58367 + 0.39862222222222222223e0_f64 * t52389 + 0.23917333333333333333e1_f64 * t52391 + 0.44291358024691358024e0_f64 * t52393;
    (t59088, t59103)
}
