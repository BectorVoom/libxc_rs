//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 712/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk712(t413: f64, t5324: f64, t829: f64, t3515: f64, t286: f64, t287: f64, t1851: f64, t3530: f64, t1262: f64, t5272: f64, t1260: f64, t1251: f64, t1847: f64, t1853: f64, t3487: f64, t3490: f64, t3499: f64, t3502: f64, t3505: f64, t3514: f64, t5300: f64, t5303: f64, t5307: f64, t5311: f64, t5316: f64, t5322: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t418 = 0.0_f64 < t413;
    let t5325 = t5324 * t829;
    let t5326 = t3515 * t5325;
    let t5329 = t286 * t287;
    let t5330 = t3530 * t1851;
    let t5331 = t5330 * t1262;
    let t5332 = t5329 * t5331;
    let t5336 = piecewise3(t418, t5272, -t5272);
    let t5337 = t1260 * t5336;
    let t5338 = t286 * t5337;
    let t5341 = -t3487 / 216.0_f64 - t3499 + t3502 / 1728.0_f64 - t3505 / 576.0_f64 - t3490 * t1847 / 216.0_f64 + t5300 / 1728.0_f64 + t3514 * t5303 / 432.0_f64 - t3514 * t5307 / 576.0_f64 - t3514 * t5311 / 288.0_f64 + t1251 * t5316 / 288.0_f64 + t3490 * t1853 / 72.0_f64 - t5322 / 576.0_f64 - t3514 * t5326 / 576.0_f64 + t1251 * t5332 / 96.0_f64 - t1251 * t5338 / 192.0_f64;
    (t5325, t5329, t5330, t5331, t5336, t5337, t5341)
}
