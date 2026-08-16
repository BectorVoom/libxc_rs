//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1784/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1784<F: Float>(t17396: F, t1791: F, t21014: F, t24729: F, t24731: F, t24734: F, t24741: F, t24753: F, t24840: F, t3671: F, t371: F, t372: F, t3720: F, t482: F, t5331: F, t5340: F, t57710: F, t59411: F, t70112: F, t70133: F, t82859: F, t83114: F, t83158: F, t89808: F) -> F {
    let t90998 = -F::cast_from(0.34299214494455789578e-2_f64) * t83158 - F::cast_from(0.20325460441158986416e-2_f64) * t70112 - F::cast_from(0.31758531939310916276e-3_f64) * t70133 - F::cast_from(0.13719685797782315831e-1_f64) * t57710 * t24840 + F::cast_from(0.51448821741683684368e-2_f64) * t59411 * t24741 + F::cast_from(0.13719685797782315831e-1_f64) * t17396 * t24753 + F::cast_from(0.17149607247227894789e-2_f64) * t5340 * t3720 * t82859 * t24729 - F::cast_from(0.85748036236139473944e-3_f64) * t5331 * t3720 * t82859 * t24734 - F::cast_from(0.27439371595564631662e-1_f64) * t21014 * t24731 - F::cast_from(0.86891343385954666928e-1_f64) * t83114 * t1791 + F::cast_from(0.12862205435420921092e-2_f64) * t3671 * t371 * t372 * t482 * t89808;
    t90998
}
