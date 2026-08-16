//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1414/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1414(t12933: f64, t12940: f64, t1629: f64, t1636: f64, t17710: f64, t18268: f64, t2128: f64, t23253: f64, t23255: f64, t23265: f64, t23268: f64, t23272: f64, t23373: f64, t4475: f64, t4480: f64, t6222: f64, t6225: f64, t6256: f64, t633: f64, t7537: f64, t7566: f64) -> f64 {
    let t23375 = 2.0_f64 * t12933 * t7537 - 6.0_f64 * t12940 * t23265 - t1629 * t23373 - t1636 * t23255 - 2.0_f64 * t17710 * t2128 + 4.0_f64 * t18268 * t6225 + t23253 * t633 + 4.0_f64 * t23268 * t4480 + 2.0_f64 * t23272 * t4480 - t4475 * t7566 - 2.0_f64 * t6222 * t6256;
    t23375
}
