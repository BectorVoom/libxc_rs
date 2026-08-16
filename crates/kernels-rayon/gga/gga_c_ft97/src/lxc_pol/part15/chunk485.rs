//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 485/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk485(t4950: f64, t4952: f64, t1096: f64, t1127: f64, t680: f64, t200: f64, t4939: f64) -> (f64, f64, f64) {
    let t4953 = t4950 * t4952;
    let t4957 = t680 * t1096 * t1127;
    let t4960 = t4939 * t200;
    (t4953, t4957, t4960)
}
