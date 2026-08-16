//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1339/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1339(t24215: f64, t2801: f64, t1052: f64, t1960: f64, t7817: f64, t2208: f64, t3459: f64, t5559: f64, t1381: f64, t3362: f64, t1383: f64, t23555: f64, t8443: f64) -> (f64, f64, f64, f64, f64) {
    let t33952 = 4.0_f64 * t24215 * t2801;
    let t33955 = 2.0_f64 * t1960 * t1052 * t7817;
    let t33958 = 6.0_f64 * t5559 * t3459 * t2208;
    let t33959 = t3362 * t1381;
    let t33961 = 2.0_f64 * t33959 * t1383;
    let t33963 = 6.0_f64 * t23555 * t8443;
    (t33952, t33955, t33958, t33961, t33963)
}
