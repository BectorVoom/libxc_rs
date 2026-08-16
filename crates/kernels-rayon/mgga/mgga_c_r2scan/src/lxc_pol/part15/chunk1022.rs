//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1022/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1022(t19790: f64, t481: f64, t1541: f64, t57: f64, t2141: f64, t3433: f64, t2146: f64, t2182: f64, t146: f64, t6091: f64, t774: f64, t537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19791 = t19790 * t481;
    let t19839 = t57 * t1541;
    let t19853 = t3433 * t2141;
    let t19865 = t2182 * t2146;
    let t19872 = t146 * t6091 * t774;
    let t19875 = t1541 * t537;
    (t19791, t19839, t19853, t19865, t19872, t19875)
}
