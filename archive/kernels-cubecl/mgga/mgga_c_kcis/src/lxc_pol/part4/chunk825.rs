//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 825/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk825<F: Float>(t413: F, t5324: F, t829: F, t3515: F, t286: F, t287: F, t1851: F, t3530: F, t1262: F, t5272: F, t1260: F, t1251: F, t1847: F, t1853: F, t3487: F, t3490: F, t3499: F, t3502: F, t3505: F, t3514: F, t5300: F, t5303: F, t5307: F, t5311: F, t5316: F, t5322: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t418 = F::cast_from(0.0_f64) < t413;
    let t5325 = t5324 * t829;
    let t5326 = t3515 * t5325;
    let t5329 = t286 * t287;
    let t5330 = t3530 * t1851;
    let t5331 = t5330 * t1262;
    let t5332 = t5329 * t5331;
    let t5336 = piecewise3::<F>(t418, t5272, -t5272);
    let t5337 = t1260 * t5336;
    let t5338 = t286 * t5337;
    let t5341 = -t3487 / F::cast_from(216.0_f64) - t3499 + t3502 / F::cast_from(1728.0_f64) - t3505 / F::cast_from(576.0_f64) - t3490 * t1847 / F::cast_from(216.0_f64) + t5300 / F::cast_from(1728.0_f64) + t3514 * t5303 / F::cast_from(432.0_f64) - t3514 * t5307 / F::cast_from(576.0_f64) - t3514 * t5311 / F::cast_from(288.0_f64) + t1251 * t5316 / F::cast_from(288.0_f64) + t3490 * t1853 / F::cast_from(72.0_f64) - t5322 / F::cast_from(576.0_f64) - t3514 * t5326 / F::cast_from(576.0_f64) + t1251 * t5332 / F::cast_from(96.0_f64) - t1251 * t5338 / F::cast_from(192.0_f64);
    (t5325, t5326, t5329, t5330, t5331, t5332, t5336, t5337, t5338, t5341)
}
