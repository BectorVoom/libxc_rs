//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 827/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk827<F: Float>(t10469: F, t2482: F, t9267: F, t2476: F, t26922: F, t9438: F, t10268: F, t4391: F, t549: F, t12996: F, t18067: F, t2365: F, t31586: F) -> (F, F, F, F, F) {
    let t41612 = t9267 * t10469 * t2482;
    let t41615 = t2476 * t9438 * t26922;
    let t41616 = F::cast_from(0.15976219147466979032e-1_f64) * t41615;
    let t41618 = t4391 * t549 * t10268;
    let t41619 = F::cast_from(0.11916829983950142223e0_f64) * t41618;
    let t41623 = t18067 * t12996;
    let t41624 = F::cast_from(0.59584149919750711116e-1_f64) * t41623;
    let t41626 = t4391 * t2365 * t31586;
    (t41612, t41616, t41619, t41624, t41626)
}
