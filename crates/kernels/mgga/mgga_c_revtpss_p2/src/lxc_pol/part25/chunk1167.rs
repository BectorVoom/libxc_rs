//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1167/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1167<F: Float>(t116: F, t6982: F, t10416: F, t1936: F, t13435: F, t2322: F, t7002: F, t13440: F, t5523: F, t112: F, t239: F, t624: F, t655: F) -> (F, F, F, F, F, F, F, F) {
    let t25805 = t6982 * t116;
    let t25812 = F::cast_from(2.0_f64) * t10416 * t1936;
    let t25814 = F::cast_from(4.0_f64) * t13435 * t1936;
    let t25816 = F::cast_from(4.0_f64) * t2322 * t7002;
    let t25818 = F::cast_from(2.0_f64) * t13440 * t1936;
    let t25820 = F::cast_from(4.0_f64) * t5523 * t7002;
    let t25821 = t239 * t112;
    let t25822 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t25821;
    let t25823 = t624 * t655;
    (t25805, t25812, t25814, t25816, t25818, t25820, t25822, t25823)
}
