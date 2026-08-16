//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1009/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1009(t35616: f64, t7839: f64, t8962: f64, t8966: f64, t33953: f64, t5284: f64, t13299: f64, t31115: f64, t31276: f64, t8875: f64, t1579: f64, t2095: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35617 = 0.15724046144802076034e-2_f64 * t35616;
    let t35623 = t7839 * t8962;
    let t35624 = 0.62896184579208304136e-3_f64 * t35623;
    let t35631 = t7839 * t8966;
    let t35632 = 0.94344276868812456204e-3_f64 * t35631;
    let t35633 = t33953 * t5284;
    let t35635 = t31115 * t13299 * t35633;
    let t35636 = 0.15724046144802076034e-2_f64 * t35635;
    let t35643 = t31276 * t8875;
    let t35646 = t2095 * t1579 * t355;
    (t35617, t35624, t35632, t35633, t35636, t35643, t35646)
}
