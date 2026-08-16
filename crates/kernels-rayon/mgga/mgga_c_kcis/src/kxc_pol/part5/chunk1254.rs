//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1254/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1254(t1495: f64, t20956: f64, t1468: f64, t1464: f64, t7321: f64, t1397: f64, t1394: f64, t5672: f64, t5748: f64, t5752: f64, t5876: f64, t1489: f64, t6281: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20957 = t1495 * t20956;
    let t20958 = t1468 * t20957;
    let t20959 = t1464 * t20958;
    let t20961 = t7321 * sigma2;
    let t20962 = t20961 * t1397;
    let t20963 = t1394 * t20962;
    let t20965 = t5748 * t5672;
    let t20966 = t1464 * t20965;
    let t20969 = t5752 * t5876;
    let t20970 = t1464 * t20969;
    let t20974 = t6281 * t1489;
    (t20959, t20961, t20963, t20966, t20970, t20974)
}
