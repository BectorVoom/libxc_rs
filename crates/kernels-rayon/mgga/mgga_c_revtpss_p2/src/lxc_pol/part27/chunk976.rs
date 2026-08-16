//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 976/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk976(t11165: f64, t4915: f64, t1066: f64, t11169: f64, t247: f64, t1011: f64, t1025: f64, t1063: f64, t11802: f64, t11806: f64, t11811: f64, t11814: f64, t11818: f64, t11824: f64, t11829: f64, t11836: f64, t3177: f64, t3184: f64, t3188: f64, t3241: f64, t3248: f64, t3255: f64, t4837: f64) -> f64 {
    let t11839 = t4915 * t11165;
    let t11845 = t247 * t1066 * t11169;
    let t11850 = 0.57165357490759649295e-3_f64 * t11802 + 0.12862205435420921092e-2_f64 * t4837 * t11806 - 0.21437009059034868486e-3_f64 * t1025 * t11811 + 0.45732285992607719436e-2_f64 * t11814 + 0.14291339372689912324e-3_f64 * t11818 + 7.0_f64 / 648.0_f64 * t1011 * t11824 - t1011 * t11829 / 36.0_f64 - t3241 * t3248 / 36.0_f64 - t3241 * t3255 / 27.0_f64 + t1011 * t11836 / 72.0_f64 - t1011 * t11839 / 48.0_f64 + 0.42874018118069736972e-3_f64 * t3188 * t3177 + 0.14291339372689912324e-3_f64 * t1063 * t11845 + 0.7145669686344956162e-3_f64 * t3188 * t3184;
    t11850
}
