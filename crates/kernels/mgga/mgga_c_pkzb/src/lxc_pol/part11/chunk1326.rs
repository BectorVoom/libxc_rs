//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1326/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1326<F: Float>(t10044: F, t10047: F, t10054: F, t10083: F, t10085: F, t10089: F, t10092: F, t10094: F, t10189: F, t11404: F, t11445: F, t11456: F, t18661: F, t2380: F, t2381: F, t27104: F, t28128: F, t28166: F, t28174: F, t3185: F, t3186: F, t3206: F, t3913: F, t3919: F, t394: F, t406: F, t8380: F, t919: F, t921: F) -> F {
    let t32137 = -F::cast_from(0.12862205435420921092e-2_f64) * t2380 * t2381 * t3919 * t10054 + F::cast_from(0.12862205435420921092e-2_f64) * t3185 * t406 * t27104 * t11404 + F::cast_from(0.12862205435420921092e-2_f64) * t3185 * t406 * t10083 * t10092 + F::cast_from(0.12862205435420921092e-2_f64) * t3206 * t2381 * t3913 * t10054 - F::cast_from(0.64311027177104605458e-3_f64) * t3206 * t406 * t8380 * t11456 - F::cast_from(0.64311027177104605458e-3_f64) * t3206 * t406 * t3186 * t394 * t10189 + F::cast_from(0.34299214494455789577e-2_f64) * t10047 * t10085 - F::cast_from(0.51448821741683684368e-2_f64) * t2380 * t18661 * t11445 * t919 * t921 + F::cast_from(0.13719685797782315831e-1_f64) * t10044 * t10089 - F::cast_from(0.13719685797782315831e-1_f64) * t10044 * t10094 + F::cast_from(0.14291339372689912324e-3_f64) * t28128 - t28166 / F::new(72.0) + t28174 / F::new(144.0);
    t32137
}
