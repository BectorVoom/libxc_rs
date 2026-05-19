//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1079/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1079<F: Float>(t1165: F, t5549: F, t7351: F, t7575: F, t5544: F, t30260: F, t34053: F, t34054: F, t34059: F, t36916: F, t36920: F, t38929: F, t38934: F, t38937: F, t38939: F, t38942: F, t38946: F, t38950: F, t38954: F, t38958: F, t38960: F) -> F {
    let t38964 = t7575 * t1165 * t7351 * t5549;
    let t38968 = t7575 * t1165 * t7351 * t5544;
    let t38970 = t36916 - F::cast_from(0.3572834843172478081e-3_f64) * t38929 - t34053 - F::cast_from(0.26416397523267487737e-1_f64) * t34054 - F::cast_from(0.21437009059034868486e-3_f64) * t38934 - t36920 - F::cast_from(0.6988464953245367126e-2_f64) * t30260 + F::cast_from(0.34299214494455789578e-1_f64) * t38937 + F::cast_from(0.18868855373762491241e-2_f64) * t38939 + t38942 / F::new(32.0) + F::cast_from(0.12579236915841660827e-2_f64) * t34059 + F::cast_from(0.21437009059034868486e-2_f64) * t38946 + F::cast_from(0.21437009059034868486e-2_f64) * t38950 + F::cast_from(0.21437009059034868486e-2_f64) * t38954 + F::cast_from(0.14291339372689912324e-2_f64) * t38958 - F::cast_from(0.47172138434406228102e-2_f64) * t38960 - F::cast_from(0.47172138434406228102e-2_f64) * t38964 - F::cast_from(0.47172138434406228102e-2_f64) * t38968;
    t38970
}
