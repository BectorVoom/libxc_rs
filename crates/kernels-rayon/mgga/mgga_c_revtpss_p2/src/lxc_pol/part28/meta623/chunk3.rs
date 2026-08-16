//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2210/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2210(t100006: f64, t100008: f64, t100019: f64, t100024: f64, t100025: f64, t100030: f64, t12116: f64, t12160: f64, t15703: f64, t16022: f64, t16091: f64, t16205: f64, t27492: f64, t27498: f64, t3120: f64, t3299: f64, t4896: f64, t4902: f64, t7132: f64, t93555: f64, t93564: f64) -> f64 {
    let t100035 = t100006 + 0.11433071498151929859e-2_f64 * t100008 * t16091 + 0.17149607247227894789e-2_f64 * t12116 * t27492 * t4896 - 0.85748036236139473944e-3_f64 * t12160 * t27492 * t4902 - 0.42874018118069736972e-3_f64 * t27498 * t16022 - 0.91464571985215438873e-2_f64 * t3299 * t100019 * t4896 + t100024 - 0.85748036236139473944e-3_f64 * t100025 * t3120 + 0.47637797908966374413e-3_f64 * t7132 * t16205 - 0.11433071498151929859e-2_f64 * t100030 * t15703 - 0.1270341277572436651e-3_f64 * t93555 + 0.28582678745379824648e-3_f64 * t93564;
    t100035
}
