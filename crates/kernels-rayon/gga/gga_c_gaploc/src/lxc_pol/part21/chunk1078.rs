//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1078/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1078(t22623: f64, t8502: f64, t1980: f64, t8520: f64, t296: f64, t8720: f64, t1: f64, t787: f64, t2021: f64, t8774: f64, t1022: f64, t5514: f64) -> (f64, f64, f64, f64, f64) {
    let t25070 = t22623 * t8502;
    let t25177 = t1980 * t8520;
    let t25191 = t296 * t8720;
    let t25193 = t787 * t25191 * t1;
    let t25198 = t2021 * t8774;
    let t25260 = t5514 * t1022;
    (t25070, t25177, t25193, t25198, t25260)
}
