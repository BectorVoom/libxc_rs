//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1032/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1032(t1083: f64, t137: f64, t34368: f64, t4257: f64, t1511: f64, t2020: f64, t31146: f64, t4487: f64, t7815: f64, t2030: f64, t5160: f64, t7440: f64, t8631: f64) -> (f64, f64, f64, f64, f64) {
    let t34369 = t1083 * t137;
    let t34371 = t34368 * t34369 * t4257;
    let t34382 = t2020 * t1511;
    let t34385 = t31146 * t7815 * t4487;
    let t34388 = t2030 * t7815 * t5160;
    let t34390 = t7440 * t8631;
    (t34371, t34382, t34385, t34388, t34390)
}
