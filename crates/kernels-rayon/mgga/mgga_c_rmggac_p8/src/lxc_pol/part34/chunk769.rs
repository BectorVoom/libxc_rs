//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 769/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk769(t446: f64, t515: f64, t570: f64, t14125: f64, t68421: f64, t2367: f64, t3351: f64, t498: f64, t7231: f64, t14117: f64, t68448: f64, t68455: f64, t9045: f64) -> (f64, f64, f64, f64) {
    let t73889 = t515 * t570 * t446;
    let t73891 = t68421 * t14125 * t73889;
    let t73896 = t3351 * t7231 * t515 * t2367 * t498;
    let t73899 = t68448 * t14117 * t73889;
    let t73902 = t68455 * t14117 * t9045;
    (t73891, t73896, t73899, t73902)
}
