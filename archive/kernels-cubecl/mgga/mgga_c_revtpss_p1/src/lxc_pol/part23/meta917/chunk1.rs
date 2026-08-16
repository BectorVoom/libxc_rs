//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2958/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2958<F: Float>(t23696: F, t3022: F, t15537: F, t6206: F, t981: F, t19049: F, t4725: F, t23451: F, t41235: F, t41238: F, t972: F, t23446: F) -> (F, F, F, F, F) {
    let t78446 = F::cast_from(0.5848223622634646207e0_f64) * t3022 * t23696;
    let t78449 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t15537 * t6206;
    let t78451 = F::cast_from(0.35089341735807877242e1_f64) * t19049 * t4725;
    let t78456 = F::cast_from(0.91082604192152556044e5_f64) * t981 * t41235 * t23451 * t41238 * t972;
    let t78458 = F::cast_from(0.35089341735807877242e1_f64) * t3022 * t23446;
    (t78446, t78449, t78451, t78456, t78458)
}
