//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1201/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1201(t6861: f64, t8085: f64, t102219: f64, t102225: f64, t102249: f64, t109450: f64, t109453: f64, t109455: f64, t109458: f64, t109460: f64, t109488: f64, t109505: f64, t2097: f64, t22953: f64, t25924: f64, t27837: f64, t30262: f64, t543: f64, t6895: f64, t7295: f64, t7301: f64, t96257: f64) -> (f64, f64) {
    let t115107 = t8085 * t6861;
    let t115126 = -0.43368140941025997312e-1_f64 * t109450 + 0.57824187921367996415e-1_f64 * t102219 + 0.77108554593144223218e-1_f64 * t109453 + 0.38554277296572111609e-1_f64 * t109455 - 0.21684070470512998656e-1_f64 * t109458 + 0.38554277296572111609e-1_f64 * t109460 - 0.10281140612419229763e-1_f64 * t102225 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t115107 * t543 + 0.13010442282307799193e1_f64 * t27837 * t30262 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2097 * t22953 * t543 - t96257 - 0.21951497276451705329e-1_f64 * t102249 - 0.16463622957338778996e-1_f64 * t109488 - 0.78062653693846795158e1_f64 * t7295 * t25924 * t8085 * t6895 + 0.32927245914677557992e-1_f64 * t109505;
    (t115107, t115126)
}
