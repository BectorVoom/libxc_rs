//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1783/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1783<F: Float>(t1774: F, t24633: F, t17401: F, t247: F, t24744: F, t24753: F, t24846: F, t3604: F, t3719: F, t3720: F, t44551: F, t5384: F, t5391: F, t57660: F, t6640: F, t6690: F, t70032: F, t70995: F, t71081: F, t83018: F, t83047: F, t83067: F, t89978: F) -> (F, F) {
    let t90926 = t24633 * t1774;
    let t90946 = -F::cast_from(0.30488190661738479624e-1_f64) * t5391 * t24846 - F::cast_from(0.3861837483820207419e-1_f64) * t83018 + F::cast_from(0.17149607247227894789e-2_f64) * t5384 * t247 * t3719 * t90926 + F::cast_from(0.3811023832717309953e-3_f64) * t70032 - F::cast_from(0.18292914397043087775e-1_f64) * t57660 * t24744 - F::cast_from(0.11433071498151929859e-2_f64) * t83047 + F::cast_from(0.19055119163586549765e-2_f64) * t83067 + F::cast_from(0.51448821741683684368e-2_f64) * t44551 * t3720 * t89978 * t3604 + F::cast_from(0.27439371595564631662e-1_f64) * t71081 * t6690 - F::cast_from(0.25724410870841842184e-2_f64) * t17401 * t24753 - F::cast_from(0.57927562257303111285e-1_f64) * t70995 * t6640;
    (t90926, t90946)
}
