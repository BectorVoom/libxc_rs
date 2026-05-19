//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 863/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk863<F: Float>(t1415: F, t42148: F, t4446: F, t10547: F, t9333: F, t12868: F, t1580: F, t12806: F, t1562: F, t4614: F, t10533: F, t20796: F, t41738: F) -> (F, F, F, F, F) {
    let t42388 = F::cast_from(0.25025342966295298669e1_f64) * t1415 * t42148 * t4446;
    let t42390 = F::cast_from(0.50050685932590597338e1_f64) * t10547 * t9333;
    let t42392 = F::cast_from(0.11502877786176224903e2_f64) * t1580 * t12868;
    let t42395 = F::cast_from(0.82820720060468819301e2_f64) * t1562 * t4614 * t12806;
    let t42398 = F::cast_from(0.27606906686822939767e2_f64) * t20796 * t10533 * t41738;
    (t42388, t42390, t42392, t42395, t42398)
}
