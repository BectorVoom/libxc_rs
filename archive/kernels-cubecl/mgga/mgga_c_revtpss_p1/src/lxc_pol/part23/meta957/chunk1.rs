//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3202/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3202<F: Float>(t59337: F, t59339: F, t71827: F, t71845: F, t71859: F, t71880: F, t71883: F, t71886: F, t71908: F, t71920: F, t71928: F, t1256: F, t24684: F) -> (F, F) {
    let t84078 = -F::cast_from(0.28582678745379824648e-3_f64) * t71827 + F::cast_from(0.17149607247227894789e-2_f64) * t71845 + F::cast_from(0.45732285992607719436e-2_f64) * t71859 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t71880 + t71883 / F::cast_from(216.0_f64) + F::cast_from(0.57165357490759649295e-3_f64) * t71886 + t59337 - t59339 - F::cast_from(0.95275595817932748825e-3_f64) * t71908 + F::cast_from(0.45732285992607719436e-2_f64) * t71920 + t71928 / F::cast_from(432.0_f64);
    let t84082 = t24684 * t1256;
    (t84078, t84082)
}
