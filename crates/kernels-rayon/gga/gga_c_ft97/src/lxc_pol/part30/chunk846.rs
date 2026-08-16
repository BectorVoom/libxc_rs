//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 846/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk846(t1168: f64, t7484: f64, t729: f64, t762: f64, t1131: f64, t7546: f64, t2568: f64, t10052: f64, t242: f64, t24737: f64, t6921: f64, t13885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35594 = t7484 * t1168;
    let t35596 = t729 * t762 * t35594;
    let t35599 = t7546 * t1131;
    let t35601 = t729 * t2568 * t35599;
    let t35604 = t7546 * t1168;
    let t35605 = t10052 * t35604;
    let t35606 = t242 * t35605;
    let t35609 = t24737 * t6921;
    let t35610 = t13885 * t35609;
    (t35594, t35596, t35599, t35601, t35604, t35605, t35606, t35609, t35610)
}
