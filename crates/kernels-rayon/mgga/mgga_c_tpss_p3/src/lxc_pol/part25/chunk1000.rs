//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1000/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1000(t12742: f64, t13613: f64, t13615: f64, t13616: f64, t13617: f64, t13621: f64, t13622: f64, t7945: f64, t7954: f64, t7960: f64, t7972: f64, t7975: f64, t9886: f64, t9900: f64, t9903: f64, t9906: f64) -> f64 {
    let t13804 = t13613 + t7945 - t13615 + t9886 - t13616 + t13617 + t9900 + t9903 - t9906 - t13621 - t7954 + t12742 + t13622 - t7960 + t7972 + t7975;
    t13804
}
