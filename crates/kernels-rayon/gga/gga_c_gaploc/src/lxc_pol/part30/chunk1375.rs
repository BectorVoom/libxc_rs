//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1375/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1375(t1424: f64, t2875: f64, t544: f64, t6540: f64, t10406: f64, t30575: f64, t30578: f64, t30607: f64, t30629: f64, t30631: f64, t30633: f64, t30644: f64, t30647: f64, t30650: f64, t34431: f64, t34435: f64, t34436: f64, t34442: f64, t34445: f64, t4849: f64) -> f64 {
    let t34449 = 0.79445533226334281486e-1_f64 * t544 * t6540 * t2875 * t1424;
    let t34450 = t34431 + t34435 - t34436 - t30575 + t30578 - 0.1022478025437886658e1_f64 * t4849 * t10406 + t34442 + t30607 + t30629 - t30631 + t30633 + t30644 - t30647 + t30650 - t34445 - t34449;
    t34450
}
