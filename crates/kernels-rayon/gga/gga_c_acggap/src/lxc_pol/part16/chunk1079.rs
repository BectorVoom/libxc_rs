//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1079/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1079(t1165: f64, t5549: f64, t7351: f64, t7575: f64, t5544: f64, t30260: f64, t34053: f64, t34054: f64, t34059: f64, t36916: f64, t36920: f64, t38929: f64, t38934: f64, t38937: f64, t38939: f64, t38942: f64, t38946: f64, t38950: f64, t38954: f64, t38958: f64, t38960: f64) -> f64 {
    let t38964 = t7575 * t1165 * t7351 * t5549;
    let t38968 = t7575 * t1165 * t7351 * t5544;
    let t38970 = t36916 - 0.3572834843172478081e-3_f64 * t38929 - t34053 - 0.26416397523267487737e-1_f64 * t34054 - 0.21437009059034868486e-3_f64 * t38934 - t36920 - 0.6988464953245367126e-2_f64 * t30260 + 0.34299214494455789578e-1_f64 * t38937 + 0.18868855373762491241e-2_f64 * t38939 + t38942 / 32.0_f64 + 0.12579236915841660827e-2_f64 * t34059 + 0.21437009059034868486e-2_f64 * t38946 + 0.21437009059034868486e-2_f64 * t38950 + 0.21437009059034868486e-2_f64 * t38954 + 0.14291339372689912324e-2_f64 * t38958 - 0.47172138434406228102e-2_f64 * t38960 - 0.47172138434406228102e-2_f64 * t38964 - 0.47172138434406228102e-2_f64 * t38968;
    t38970
}
