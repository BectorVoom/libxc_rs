//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2858/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2858<F: Float>(t162: F, t4403: F, t61037: F, t61315: F, t18259: F, t18559: F, t40172: F, t62274: F, t62276: F, t62282: F, t50888: F, t62300: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77020 = F::new(36.0) * t61037 * t162 * t4403;
    let t77021 = F::new(72.0) * t61315;
    let t77023 = F::new(72.0) * t18259 * t18559;
    let t77024 = F::cast_from(0.10254018858216406658e4_f64) * t40172;
    let t77025 = F::new(72.0) * t62274;
    let t77026 = F::cast_from(0.35089341735807877242e1_f64) * t62276;
    let t77027 = F::new(72.0) * t62282;
    let t77028 = F::cast_from(0.10526802520742363173e2_f64) * t50888;
    let t77029 = F::new(3.0) * t62300;
    (t77020, t77021, t77023, t77024, t77025, t77026, t77027, t77028, t77029)
}
