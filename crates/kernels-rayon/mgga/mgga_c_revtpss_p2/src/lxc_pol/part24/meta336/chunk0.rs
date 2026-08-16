//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1173/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1173(t1579: f64, t6071: f64, t2770: f64, t6048: f64, t11008: f64, t10987: f64, t11017: f64, t11040: f64, t15018: f64, t15063: f64, t1580: f64, t18798: f64, t18800: f64, t18806: f64, t18812: f64, t18815: f64, t18822: f64, t18826: f64, t18828: f64, t865: f64) -> (f64, f64, f64, f64, f64) {
    let t23403 = t1579 * t6071;
    let t23404 = t2770 * t23403;
    let t23413 = t6048 * t1579;
    let t23414 = t11008 * t23413;
    let t23420 = 0.34697458558045176417e-2_f64 * t15018 - t10987 - 0.29272321618148349057e-1_f64 * t18798 + 0.39512695097613069591e1_f64 * t865 * t23404 + t11017 + 0.58544643236296698113e-1_f64 * t18806 + 0.16463622957338778996e-1_f64 * t18812 + 0.32927245914677557992e-1_f64 * t18815 + 0.29272321618148349057e-1_f64 * t18822 + 0.21951497276451705329e-1_f64 * t15063 - t11040 - 0.32927245914677557992e-1_f64 * t18826 - 0.39512695097613069591e1_f64 * t865 * t23414 - 0.58544643236296698113e-1_f64 * t18828 - 0.19756347548806534796e1_f64 * t18800 * t1580;
    (t23403, t23404, t23413, t23414, t23420)
}
