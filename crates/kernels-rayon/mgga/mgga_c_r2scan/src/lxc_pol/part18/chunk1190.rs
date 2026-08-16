//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1190/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1190(t11780: f64, t2201: f64, t3602: f64, t10856: f64, t9423: f64, t11643: f64, t25983: f64, t261: f64, t3304: f64, t9476: f64, t37759: f64, t37823: f64, t37834: f64, t37835: f64, t37838: f64, t37841: f64, t41518: f64, t41519: f64) -> f64 {
    let t43215 = t2201 * t11780 * t3602;
    let t43217 = t10856 * t9423;
    let t43219 = t25983 * t11643;
    let t43225 = t3304 * t261 * t9476;
    let t43227 = -0.59512461497092438715e-1_f64 * t37759 + 0.43663693315433241792e-2_f64 * t43215 - 0.48787202696913915093e-2_f64 * t43217 - t41518 + t41519 - 0.13099107994629972538e-1_f64 * t43219 + t37823 + t37834 + 0.29272321618148349056e-1_f64 * t37835 + 0.22511059664845582436e0_f64 * t37838 + 0.67533178994536747308e0_f64 * t37841 + 0.34672886960217074253e0_f64 * t43225;
    t43227
}
