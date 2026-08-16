//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1043/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1043(t1966: f64, t43842: f64, t590: f64, t1890: f64, t43107: f64, t10948: f64, t11016: f64, t13012: f64, t2087: f64, t4614: f64, t3267: f64, t8634: f64) -> (f64, f64, f64, f64, f64) {
    let t43844 = t1966 * t43842 * t590;
    let t43849 = 0.25561950635947166451e1_f64 * t1966 * t1890 * t43107 * t590;
    let t43854 = t10948 * t11016;
    let t43858 = 0.92023022289409799224e1_f64 * t2087 * t4614 * t13012;
    let t43861 = 0.35750489951850426669e0_f64 * t3267 * t8634;
    (t43844, t43849, t43854, t43858, t43861)
}
