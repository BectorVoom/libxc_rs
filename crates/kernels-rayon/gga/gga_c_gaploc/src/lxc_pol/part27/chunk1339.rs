//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1339/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1339(t34830: f64, t18736: f64, t2365: f64, t25575: f64, t25735: f64, t7025: f64, t10402: f64, t20675: f64, t204: f64, t2476: f64, t34371: f64, t10310: f64, t1429: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34831 = 0.59584149919750711116e-1_f64 * t34830;
    let t34838 = t18736 * t2365 * t25575;
    let t34839 = 0.29792074959875355558e-1_f64 * t34838;
    let t34841 = t7025 * t2365 * t25735;
    let t34842 = 0.29792074959875355558e-1_f64 * t34841;
    let t34854 = t20675 * t10402;
    let t34855 = 0.38342925953920749676e0_f64 * t34854;
    let t34860 = 0.92023022289409799224e1_f64 * t2476 * t204 * t34371;
    let t34862 = t1429 * t549 * t10310;
    (t34831, t34839, t34842, t34855, t34860, t34862)
}
