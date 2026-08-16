//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 927/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk927(t1991: f64, t43838: f64, t590: f64, t1890: f64, t1966: f64, t43107: f64, t13012: f64, t2087: f64, t4614: f64, t3267: f64, t8634: f64, t13033: f64, t5748: f64) -> (f64, f64, f64, f64, f64) {
    let t43841 = 0.1022478025437886658e1_f64 * t1991 * t43838 * t590;
    let t43849 = 0.25561950635947166451e1_f64 * t1966 * t1890 * t43107 * t590;
    let t43858 = 0.92023022289409799224e1_f64 * t2087 * t4614 * t13012;
    let t43861 = 0.35750489951850426669e0_f64 * t3267 * t8634;
    let t43864 = 0.36809208915763919689e2_f64 * t5748 * t4614 * t13033;
    (t43841, t43849, t43858, t43861, t43864)
}
