//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 732/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk732(t2: f64, t386: f64, t481: f64, t506: f64, t6069: f64, t2106: f64, t776: f64, t162: f64, t9: f64, t2097: f64, t2105: f64, t265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6072 = t506 * t2 * t386 * t481;
    let t6073 = t6069 * t6072;
    let t6075 = t776 * t2106;
    let t6077 = t162 * t162;
    let t6079 = 1.0_f64 / t9 / t6077;
    let t6082 = t2097 * t6079 * t265 * t2105;
    (t6072, t6073, t6075, t6077, t6079, t6082)
}
