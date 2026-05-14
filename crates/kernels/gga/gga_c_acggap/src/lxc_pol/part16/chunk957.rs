//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 957/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk957<F: Float>(t1181: F, t5549: F, t604: F, t7575: F, t5544: F, t1849: F, t1992: F, t30154: F, t7586: F, t30219: F, t9653: F, t1165: F, t7351: F, t30260: F, t34053: F, t34054: F, t34059: F, t36916: F, t36920: F, t38929: F, t38934: F, t38937: F, t38939: F, t38942: F, t38946: F) -> (F, F) {
    let t38950 = t7575 * t1181 * t604 * t5549;
    let t38954 = t7575 * t1181 * t604 * t5544;
    let t38956 = t1992 * t1849;
    let t38958 = t30154 * t7586 * t38956;
    let t38960 = t30219 * t9653;
    let t38964 = t7575 * t1165 * t7351 * t5549;
    let t38968 = t7575 * t1165 * t7351 * t5544;
    let t38970 = t36916 - 0.3572834843172478081e-3 * t38929 - t34053 - 0.26416397523267487737e-1 * t34054 - 0.21437009059034868486e-3 * t38934 - t36920 - 0.6988464953245367126e-2 * t30260 + 0.34299214494455789578e-1 * t38937 + 0.18868855373762491241e-2 * t38939 + t38942 / 32.0 + 0.12579236915841660827e-2 * t34059 + 0.21437009059034868486e-2 * t38946 + 0.21437009059034868486e-2 * t38950 + 0.21437009059034868486e-2 * t38954 + 0.14291339372689912324e-2 * t38958 - 0.47172138434406228102e-2 * t38960 - 0.47172138434406228102e-2 * t38964 - 0.47172138434406228102e-2 * t38968;
    (t38956, t38970)
}
