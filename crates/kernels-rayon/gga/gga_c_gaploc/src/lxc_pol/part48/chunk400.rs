//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 400/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk400(t1445: f64, t3483: f64, t813: f64, t1022: f64, t123: f64) -> (f64, f64, f64) {
    let t3484 = t1445 * t3483;
    let t3486 = 0.46011511144704899612e1_f64 * t813 * t3484;
    let t3487 = t1022 * t123;
    (t3484, t3486, t3487)
}
