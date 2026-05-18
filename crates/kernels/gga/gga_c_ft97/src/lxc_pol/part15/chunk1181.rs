//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1181/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1181<F: Float>(t1208: F, t21253: F, t83210: F, t22138: F, t7853: F, t2035: F, t5266: F, t5284: F, t1111: F, t1200: F, t14729: F, t19049: F, t19053: F, t22067: F, t22068: F, t22069: F, t2691: F, t4099: F, t4113: F, t54859: F, t7003: F, t70497: F, t90172: F, t90186: F, t90201: F) -> (F, F) {
    let t90208 = t83210 * t21253 * t1208;
    let t90216 = t7853 * t22138;
    let t90224 = t2035 * t5266 * t5284;
    let t90234 = F::new(0.86903958837283218463e0) * t2691 * t90201 + F::new(0.13035593825592482769e1) * t7003 * t90208 - F::new(0.43451979418641609231e0) * t4113 * t90208 - F::new(48.0) * t2691 * t54859 * t22067 + F::new(0.91821883503738212655e2) * t19049 * t90216 - F::new(0.91821883503738212655e2) * t19053 * t90216 - F::new(0.22955470875934553164e2) * t4099 * t90172 - F::new(0.35032929183548774392e2) * t70497 * t90224 + F::new(0.14498192132169191472e2) * t1200 * t22068 * t1111 - F::new(0.14498192132169191472e2) * t22069 * t1111 - F::new(0.45910941751869106328e2) * t14729 * t90186;
    (t90224, t90234)
}
