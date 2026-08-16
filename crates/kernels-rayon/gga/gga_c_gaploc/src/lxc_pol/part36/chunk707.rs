//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 707/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk707(t13023: f64, t1457: f64, t2103: f64, t3040: f64, t3271: f64, t11001: f64, t955: f64, t10948: f64, t3470: f64, t3209: f64, t8604: f64, t1445: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13024 = t1457 * t13023;
    let t13026 = 0.71500979903700853338e0_f64 * t2103 * t13024;
    let t13028 = 0.35750489951850426669e0_f64 * t3271 * t3040;
    let t13029 = t955 * t11001;
    let t13031 = t10948 * t3470;
    let t13033 = t8604 * t3209;
    let t13034 = t1445 * t13033;
    (t13024, t13026, t13028, t13029, t13031, t13033, t13034)
}
