//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1181/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1181(t1208: f64, t21253: f64, t83210: f64, t22138: f64, t7853: f64, t2035: f64, t5266: f64, t5284: f64, t1111: f64, t1200: f64, t14729: f64, t19049: f64, t19053: f64, t22067: f64, t22068: f64, t22069: f64, t2691: f64, t4099: f64, t4113: f64, t54859: f64, t7003: f64, t70497: f64, t90172: f64, t90186: f64, t90201: f64) -> (f64, f64) {
    let t90208 = t83210 * t21253 * t1208;
    let t90216 = t7853 * t22138;
    let t90224 = t2035 * t5266 * t5284;
    let t90234 = 0.86903958837283218463e0_f64 * t2691 * t90201 + 0.13035593825592482769e1_f64 * t7003 * t90208 - 0.43451979418641609231e0_f64 * t4113 * t90208 - 48.0_f64 * t2691 * t54859 * t22067 + 0.91821883503738212655e2_f64 * t19049 * t90216 - 0.91821883503738212655e2_f64 * t19053 * t90216 - 0.22955470875934553164e2_f64 * t4099 * t90172 - 0.35032929183548774392e2_f64 * t70497 * t90224 + 0.14498192132169191472e2_f64 * t1200 * t22068 * t1111 - 0.14498192132169191472e2_f64 * t22069 * t1111 - 0.45910941751869106328e2_f64 * t14729 * t90186;
    (t90224, t90234)
}
