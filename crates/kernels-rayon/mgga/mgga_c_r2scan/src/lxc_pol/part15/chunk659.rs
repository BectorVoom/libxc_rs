//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 659/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk659(t1391: f64, t502: f64, t1390: f64, t386: f64, t518: f64, t385: f64, t4715: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let t4735 = t1391 * t502;
    let t4736 = t1390 * t4735;
    let t4738 = t386 * t518;
    let t4739 = t385 * t4738;
    let t4741 = t5 * t4715;
    (t4735, t4736, t4738, t4739, t4741)
}
