//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1118/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1118<F: Float>(t154: F, t2347: F, t3730: F, t385: F, t1220: F, t3171: F, t2099: F, t3876: F, t2395: F, t10103: F, t10108: F, t10112: F, t10117: F, t10123: F, t10132: F, t3174: F, t3181: F, t3202: F, t3214: F, t3849: F, t6379: F, t6383: F, t8275: F, t8285: F, t8317: F, t8325: F, t8428: F, t8450: F, t909: F) -> (F,) {
    let t10135 = t154 * t2347 * t3730;
    let t10136 = t385 * t10135;
    let t10138 = t1220 * t3171;
    let t10140 = t2099 * t3876;
    let t10141 = t2395 * t10140;
    let t10146 = 0.21437009059034868486e-3 * t8450 * t10103 - t3174 * t10108 / 16.0 + t3174 * t10112 / 24.0 + t3174 * t10117 / 48.0 + t6379 + 0.95275595817932748826e-4 * t6383 + 0.12862205435420921092e-2 * t8428 * t10123 - 11.0 / 108.0 * t3849 * t909 + t1220 * t3181 / 18.0 + t10132 / 144.0 - t10136 / 288.0 + t10138 / 54.0 - 0.14291339372689912324e-3 * t10141 + t8275 + 0.5081365110289746604e-3 * t8285 + t8317 + t8325 - 0.22866142996303859718e-2 * t3214 * t3202;
    (t10146,)
}
