//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1783/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1783(t1774: f64, t24633: f64, t17401: f64, t247: f64, t24744: f64, t24753: f64, t24846: f64, t3604: f64, t3719: f64, t3720: f64, t44551: f64, t5384: f64, t5391: f64, t57660: f64, t6640: f64, t6690: f64, t70032: f64, t70995: f64, t71081: f64, t83018: f64, t83047: f64, t83067: f64, t89978: f64) -> (f64, f64) {
    let t90926 = t24633 * t1774;
    let t90946 = -0.30488190661738479624e-1_f64 * t5391 * t24846 - 0.3861837483820207419e-1_f64 * t83018 + 0.17149607247227894789e-2_f64 * t5384 * t247 * t3719 * t90926 + 0.3811023832717309953e-3_f64 * t70032 - 0.18292914397043087775e-1_f64 * t57660 * t24744 - 0.11433071498151929859e-2_f64 * t83047 + 0.19055119163586549765e-2_f64 * t83067 + 0.51448821741683684368e-2_f64 * t44551 * t3720 * t89978 * t3604 + 0.27439371595564631662e-1_f64 * t71081 * t6690 - 0.25724410870841842184e-2_f64 * t17401 * t24753 - 0.57927562257303111285e-1_f64 * t70995 * t6640;
    (t90926, t90946)
}
