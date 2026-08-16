//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 679/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk679(t14048: f64, t68581: f64, t13863: f64, t14368: f64, t13822: f64, t7348: f64, t13824: f64, t14024: f64, t4517: f64, t830: f64, t14130: f64, t1985: f64, t3839: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68582 = t68581 * t14048;
    let t68602 = t14368 * t13863;
    let t68613 = t13822 * t7348;
    let t68614 = t68613 * t13824;
    let t68621 = t4517 * t830 * t14024;
    let t68622 = t14130 * t68621;
    let t68626 = t1985 * t3839;
    (t68582, t68602, t68613, t68614, t68621, t68622, t68626)
}
