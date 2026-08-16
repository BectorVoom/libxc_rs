//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1196/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1196(t10856: f64, t9236: f64, t10698: f64, t12506: f64, t12517: f64, t1584: f64, t29936: f64, t3308: f64, t574: f64, t12520: f64, t12463: f64, t2207: f64, t3336: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43692 = t10856 * t9236;
    let t43695 = t10698 * t12506;
    let t43697 = t1584 * t12517;
    let t43700 = t574 * t3308 * t29936;
    let t43702 = t1584 * t12520;
    let t43705 = t2207 * t3336 * t12463;
    (t43692, t43695, t43697, t43700, t43702, t43705)
}
