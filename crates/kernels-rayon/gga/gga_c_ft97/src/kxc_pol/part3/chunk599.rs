//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 599/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk599(t200: f64, t4977: f64, t680: f64, t2379: f64, t4960: f64, t1113: f64, t203: f64, t237: f64, t1127: f64, t3767: f64, t207: f64, t215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4978 = t4977 * t200;
    let t4979 = t680 * t4978;
    let t4982 = t2379 * t4960;
    let t4985 = t1113 * t1113;
    let t4986 = t203 * t4985;
    let t4987 = t4986 * t237;
    let t4991 = t3767 * t1127;
    let t4995 = 1.0_f64 / t207 / t215;
    (t4978, t4979, t4982, t4985, t4986, t4987, t4991, t4995)
}
