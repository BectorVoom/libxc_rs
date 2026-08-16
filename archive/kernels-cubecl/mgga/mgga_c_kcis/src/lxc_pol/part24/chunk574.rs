//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 574/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk574<F: Float>(t1260: F, t5336: F, t286: F, t1251: F, t1847: F, t1853: F, t3487: F, t3490: F, t3499: F, t3502: F, t3505: F, t3514: F, t5300: F, t5303: F, t5307: F, t5311: F, t5316: F, t5322: F, t5326: F, t5332: F) -> (F, F) {
    let t5337 = t1260 * t5336;
    let t5338 = t286 * t5337;
    let t5341 = -t3487 / F::cast_from(216.0_f64) - t3499 + t3502 / F::cast_from(1728.0_f64) - t3505 / F::cast_from(576.0_f64) - t3490 * t1847 / F::cast_from(216.0_f64) + t5300 / F::cast_from(1728.0_f64) + t3514 * t5303 / F::cast_from(432.0_f64) - t3514 * t5307 / F::cast_from(576.0_f64) - t3514 * t5311 / F::cast_from(288.0_f64) + t1251 * t5316 / F::cast_from(288.0_f64) + t3490 * t1853 / F::cast_from(72.0_f64) - t5322 / F::cast_from(576.0_f64) - t3514 * t5326 / F::cast_from(576.0_f64) + t1251 * t5332 / F::cast_from(96.0_f64) - t1251 * t5338 / F::cast_from(192.0_f64);
    (t5337, t5341)
}
