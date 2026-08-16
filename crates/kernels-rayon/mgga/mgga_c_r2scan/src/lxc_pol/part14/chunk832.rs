//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 832/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk832(t2124: f64, t495: f64, t7503: f64, t6217: f64, t7460: f64, t1593: f64, t2562: f64, t360: f64, t6359: f64, t920: f64, t1553: f64, t6363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7509 = t2124 * t7503 * t495;
    let t7512 = t6217 * t7460;
    let t7513 = t2562 * t1593;
    let t7514 = t360 * t7513;
    let t7517 = t6359 * t920;
    let t7518 = t6363 * t1553;
    (t7509, t7512, t7513, t7514, t7517, t7518)
}
