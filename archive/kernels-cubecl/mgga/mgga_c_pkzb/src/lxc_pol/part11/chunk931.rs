//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 931/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk931<F: Float>(t10140: F, t2395: F, t10103: F, t10108: F, t10112: F, t10117: F, t10123: F, t10132: F, t10136: F, t10138: F, t1220: F, t3174: F, t3181: F, t3202: F, t3214: F, t3849: F, t6379: F, t6383: F, t8275: F, t8285: F, t8317: F, t8325: F, t8428: F, t8450: F, t909: F) -> (F, F) {
    let t10141 = t2395 * t10140;
    let t10146 = F::cast_from(0.21437009059034868486e-3_f64) * t8450 * t10103 - t3174 * t10108 / F::cast_from(16.0_f64) + t3174 * t10112 / F::cast_from(24.0_f64) + t3174 * t10117 / F::cast_from(48.0_f64) + t6379 + F::cast_from(0.95275595817932748826e-4_f64) * t6383 + F::cast_from(0.12862205435420921092e-2_f64) * t8428 * t10123 - F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t3849 * t909 + t1220 * t3181 / F::cast_from(18.0_f64) + t10132 / F::cast_from(144.0_f64) - t10136 / F::cast_from(288.0_f64) + t10138 / F::cast_from(54.0_f64) - F::cast_from(0.14291339372689912324e-3_f64) * t10141 + t8275 + F::cast_from(0.5081365110289746604e-3_f64) * t8285 + t8317 + t8325 - F::cast_from(0.22866142996303859718e-2_f64) * t3214 * t3202;
    (t10141, t10146)
}
