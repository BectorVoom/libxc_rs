//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1247/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1247<F: Float>(t1048: F, t44181: F, t44188: F, t44202: F, t44209: F, t44216: F, t44229: F, t44242: F, t44255: F, t44268: F, t44278: F, t44288: F, t44297: F, t44308: F, t44316: F, t44330: F, t44343: F, t44359: F, t44370: F, t44380: F, t44388: F, t44396: F, t44407: F, t44412: F, t44424: F, t44440: F, t44452: F, t44461: F, t44471: F, t44483: F, t44492: F, t44500: F, t44510: F, t499: F, t797: F) -> F {
    let t44519 = t1048 * t499 * (t44343 + t44255 + t44330 + t44407 + t44461 + t44452 + t44288 + t44359 + t44483 + t44440 + t44380 + t44181 + t44492 + t44396 + t44316 + t44229 + t44471 + t44202 + t44268 + t44308 + t44510 + t44297 + t44370 + t44188 + t44500 + t44424 + t44412 + t44216 + t44278 + t44209 + t44242 + t44388) * t797 / F::new(4.0);
    t44519
}
