//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 762/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk762(t4685: f64, t511: f64, t1982: f64, t7428: f64, t7434: f64, t1326: f64, t2016: f64, t7551: f64, t2049: f64, t35253: f64, t7760: f64, t2019: f64, t271: f64, t3118: f64, t641: f64) -> (f64, f64, f64, f64, f64) {
    let t35674 = t4685 * t511;
    let t35683 = t7434 * t7428 * t1982;
    let t35688 = t2016 * t7551 * t1326;
    let t35691 = t35688 * t2049 * t35253 * t7760;
    let t35696 = t2019 * t3118 * t271 * t641;
    (t35674, t35683, t35688, t35691, t35696)
}
