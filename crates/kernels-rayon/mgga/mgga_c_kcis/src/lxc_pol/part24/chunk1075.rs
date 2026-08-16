//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1075/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1075(t28093: f64, t7772: f64, t1268: f64, t1851: f64, t922: f64, t3515: f64, t5281: f64, t5310: f64, t1262: f64, t1646: f64, t26961: f64, t330: f64, t3622: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28094 = t7772 * t28093;
    let t28096 = t1268 * t1851;
    let t28097 = t28096 * t922;
    let t28098 = t3515 * t28097;
    let t28101 = t5281 * t922;
    let t28102 = t5310 * t28101;
    let t28105 = t1646 * t1262;
    let t28106 = t26961 * t28105;
    let t28107 = t3515 * t28106;
    let t28110 = t3622 * t330;
    (t28094, t28097, t28098, t28101, t28102, t28105, t28106, t28107, t28110)
}
