//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1366/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1366(t1928: f64, t5742: f64, t990: f64, t102308: f64, t102311: f64, t102313: f64, t102334: f64, t102337: f64, t102340: f64, t102348: f64, t103063: f64, t28369: f64, t28375: f64, t28388: f64, t28392: f64, t28420: f64, t7911: f64, t8155: f64, t98294: f64) -> f64 {
    let t103445 = t5742 * t1928 * t990;
    let t103459 = -0.24712962962962962964e-2_f64 * t28392 * t28420 - 0.185671721767578125e-4_f64 * t28388 * t103063 + 0.12356481481481481481e-2_f64 * t103445 * t7911 + 0.12356481481481481482e-2_f64 * t98294 * t8155 - 0.58958024691358024689e-2_f64 * t102308 + 0.11054629629629629629e-2_f64 * t102311 - 0.27802083333333333334e-2_f64 * t28369 * t28375 - 0.22109259259259259259e-2_f64 * t102313 - 0.27636574074074074073e-2_f64 * t102334 + 0.18424382716049382715e-2_f64 * t102337 - 0.16581944444444444444e-1_f64 * t102340 + 0.73697530864197530861e-2_f64 * t102348;
    t103459
}
