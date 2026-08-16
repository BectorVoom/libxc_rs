//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 923/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk923<F: Float>(t1579: F, t6071: F, t2770: F, t6048: F, t11008: F, t10987: F, t11017: F, t11040: F, t15018: F, t15063: F, t1580: F, t18798: F, t18800: F, t18806: F, t18812: F, t18815: F, t18822: F, t18826: F, t18828: F, t865: F) -> (F, F, F, F) {
    let t23403 = t1579 * t6071;
    let t23404 = t2770 * t23403;
    let t23413 = t6048 * t1579;
    let t23414 = t11008 * t23413;
    let t23420 = F::cast_from(0.34697458558045176417e-2_f64) * t15018 - t10987 - F::cast_from(0.29272321618148349057e-1_f64) * t18798 + F::cast_from(0.39512695097613069591e1_f64) * t865 * t23404 + t11017 + F::cast_from(0.58544643236296698113e-1_f64) * t18806 + F::cast_from(0.16463622957338778996e-1_f64) * t18812 + F::cast_from(0.32927245914677557992e-1_f64) * t18815 + F::cast_from(0.29272321618148349057e-1_f64) * t18822 + F::cast_from(0.21951497276451705329e-1_f64) * t15063 - t11040 - F::cast_from(0.32927245914677557992e-1_f64) * t18826 - F::cast_from(0.39512695097613069591e1_f64) * t865 * t23414 - F::cast_from(0.58544643236296698113e-1_f64) * t18828 - F::cast_from(0.19756347548806534796e1_f64) * t18800 * t1580;
    (t23404, t23413, t23414, t23420)
}
