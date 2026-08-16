//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1088/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1088(t2068: f64, t7342: f64, t8480: f64, t30782: f64, t34879: f64, t34883: f64, t34887: f64, t34891: f64, t34894: f64, t34896: f64, t34897: f64, t34901: f64, t34905: f64, t34909: f64, t34913: f64, t34916: f64, t34920: f64, t34923: f64, t34926: f64, t34929: f64) -> f64 {
    let t34933 = t2068 * t8480 * t7342;
    let t34935 = 0.42874018118069736972e-3_f64 * t34879 - 0.22921875e-1_f64 * t34883 - 0.4584375e-1_f64 * t34887 - 0.22921875e-1_f64 * t34891 + t34894 + t34896 - 0.65369791666666666667e-1_f64 * t34897 + 0.22921875e0_f64 * t34901 - t34905 / 16.0_f64 - 0.916875e-1_f64 * t34909 - 0.4584375e-1_f64 * t34913 - 0.4584375e-1_f64 * t34916 - 0.4584375e-1_f64 * t34920 - 0.4584375e-1_f64 * t34923 - 0.916875e-1_f64 * t34926 - 0.4584375e-1_f64 * t34929 - 0.916875e-1_f64 * t30782 + 0.42874018118069736972e-3_f64 * t34933;
    t34935
}
