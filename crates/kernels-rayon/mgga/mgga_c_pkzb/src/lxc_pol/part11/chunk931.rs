//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 931/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk931(t10140: f64, t2395: f64, t10103: f64, t10108: f64, t10112: f64, t10117: f64, t10123: f64, t10132: f64, t10136: f64, t10138: f64, t1220: f64, t3174: f64, t3181: f64, t3202: f64, t3214: f64, t3849: f64, t6379: f64, t6383: f64, t8275: f64, t8285: f64, t8317: f64, t8325: f64, t8428: f64, t8450: f64, t909: f64) -> (f64, f64) {
    let t10141 = t2395 * t10140;
    let t10146 = 0.21437009059034868486e-3_f64 * t8450 * t10103 - t3174 * t10108 / 16.0_f64 + t3174 * t10112 / 24.0_f64 + t3174 * t10117 / 48.0_f64 + t6379 + 0.95275595817932748826e-4_f64 * t6383 + 0.12862205435420921092e-2_f64 * t8428 * t10123 - 11.0_f64 / 108.0_f64 * t3849 * t909 + t1220 * t3181 / 18.0_f64 + t10132 / 144.0_f64 - t10136 / 288.0_f64 + t10138 / 54.0_f64 - 0.14291339372689912324e-3_f64 * t10141 + t8275 + 0.5081365110289746604e-3_f64 * t8285 + t8317 + t8325 - 0.22866142996303859718e-2_f64 * t3214 * t3202;
    (t10141, t10146)
}
