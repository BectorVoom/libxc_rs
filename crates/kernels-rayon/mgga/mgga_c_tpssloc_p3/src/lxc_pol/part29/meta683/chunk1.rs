//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2315/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2315(t27598: f64, t3535: f64, t1218: f64, t14731: f64, t14736: f64, t14740: f64, t15663: f64, t15750: f64, t2121: f64, t24736: f64, t24741: f64, t4899: f64, t4989: f64, t7331: f64, t8040: f64, t86204: f64, t86324: f64, t95410: f64, t95415: f64, t95424: f64, t95435: f64) -> f64 {
    let t95440 = t3535 * t27598;
    let t95443 = -t95410 - 0.10093189023535097714e-3_f64 * t86204 * t8040 + 0.20186378047070195428e-3_f64 * t95415 * t7331 + 5.0_f64 / 3456.0_f64 * t24736 * t4989 - t95424 + t2121 * t4899 * t14736 / 108.0_f64 + t2121 * t4899 * t14740 / 216.0_f64 + t2121 * t4899 * t14731 / 36.0_f64 - t95435 - t86324 * t15663 / 576.0_f64 + 5.0_f64 / 3456.0_f64 * t24741 * t15750 - t95440 * t1218 / 144.0_f64;
    t95443
}
