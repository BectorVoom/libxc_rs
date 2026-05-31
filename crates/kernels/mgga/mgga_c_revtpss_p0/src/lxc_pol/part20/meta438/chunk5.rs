//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1656/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1656<F: Float>(t3471: F, t1169: F, t1170: F, t1188: F, t12418: F, t12423: F, t12429: F, t12431: F, t12514: F, t12555: F, t12556: F, t3452: F, t3454: F, t3472: F, t3477: F, t3479: F, t3496: F, t3521: F, t3523: F, t43750: F, t43753: F, t43966: F, t44014: F, t44021: F, t44087: F, t45057: F, t45174: F, t45177: F, t45181: F, t45188: F, t45190: F, t45194: F, t45197: F) -> F {
    let t45205 = t3471 * t3471;
    let t45218 = F::cast_from(0.4101607543286562663e4_f64) * t45174 * t12556 - F::cast_from(0.12304822629859687989e5_f64) * t45177 * t43753 * t12555 + F::cast_from(4.0_f64) * t45181 * t1170 + F::cast_from(6.0_f64) * t12418 * t3472 + F::cast_from(0.91082604192152556044e5_f64) * t45188 * t43753 * t45190 - F::cast_from(12.0_f64) * t45194 * t3454 - F::cast_from(0.77193501593724168322e3_f64) * t45197 * t12431 + F::cast_from(24.0_f64) * t12423 * t12514 - F::cast_from(24.0_f64) * t12429 * t45057 * t1169 - F::cast_from(6.0_f64) * t3452 * t45205 * t1169 + F::cast_from(0.96491876992155210402e2_f64) * t3477 * t45205 * t3479 + t43750 - F::cast_from(0.35089341735807877242e1_f64) * t3496 * t43966 * t1188 + F::cast_from(0.51947577317044391277e2_f64) * t3521 * t43966 * t3523 - t44014 + t44021 - t44087;
    t45218
}
