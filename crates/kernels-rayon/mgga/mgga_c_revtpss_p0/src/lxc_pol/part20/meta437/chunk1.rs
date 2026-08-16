//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1649/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1649(t3453: f64, t3488: f64, t3495: f64, t1175: f64, t12485: f64, t3444: f64, t3476: f64, t1156: f64, t12469: f64, t3450: f64, t3475: f64, t426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45057 = t3453 * t3453;
    let t45061 = t3488 * t3495;
    let t45064 = t1175 * t12485;
    let t45075 = t3444 * t3476;
    let t45080 = t1156 * t12469;
    let t45085 = t426 / t3475 / t3450;
    (t45057, t45061, t45064, t45075, t45080, t45085)
}
