//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1000/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1000(t10939: f64, t10948: f64, t10969: f64, t10971: f64, t14581: f64, t14948: f64, t14951: f64, t14961: f64, t1559: f64, t18714: f64, t18720: f64, t18727: f64, t18731: f64, t18733: f64, t18739: f64, t18743: f64, t18747: f64, t18751: f64, t18763: f64, t6022: f64, t820: f64) -> f64 {
    let t23382 = -0.19756347548806534796e1_f64 * t820 * t18714 * t1559 + 0.58544643236296698113e-1_f64 * t18720 + 0.21951497276451705329e-1_f64 * t14581 - 0.29272321618148349057e-1_f64 * t18727 - 0.29272321618148349057e-1_f64 * t18731 + 0.39512695097613069591e1_f64 * t820 * t14961 * t6022 - 0.58544643236296698113e-1_f64 * t18733 + 0.16463622957338778996e-1_f64 * t18739 + 0.16463622957338778996e-1_f64 * t18743 + 0.32927245914677557992e-1_f64 * t18747 - 0.32927245914677557992e-1_f64 * t18751 + 0.34697458558045176417e-2_f64 * t14948 - 0.39029762157531132076e-1_f64 * t14951 + 0.29272321618148349057e-1_f64 * t18763 + t10939 - t10948 + t10969 - t10971;
    t23382
}
