//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 913/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk913(t23046: f64, t242: f64, t812: f64, t4184: f64, t23146: f64, t4191: f64, t4240: f64, t4250: f64, t13228: f64, t828: f64, t2628: f64, t6605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25083 = t23046 * t242;
    let t25084 = t812 * t25083;
    let t25085 = t25084 * t4184;
    let t25087 = t23146 * t4191;
    let t25089 = t23146 * t4240;
    let t25091 = t23146 * t4250;
    let t25093 = t13228 * t828;
    let t25094 = t2628 * t25093;
    let t25095 = t6605 * t25094;
    (t25085, t25087, t25089, t25091, t25093, t25095)
}
