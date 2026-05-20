//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1716/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1716<F: Float>(t12079: F, t12122: F, t12127: F, t12168: F, t16502: F, t16552: F, t16553: F, t16559: F, t16560: F, t1668: F, t19450: F, t19603: F, t23820: F, t24083: F, t24090: F, t24135: F, t24141: F, t3304: F, t3318: F, t43520: F, t43524: F, t4893: F, t4981: F, t4982: F, t55732: F, t6299: F, t80264: F, t88794: F, t88804: F) -> F {
    let t89603 = -F::cast_from(0.15805078039045227836e2_f64) * t43520 * t88794 * t12168 + F::cast_from(0.15805078039045227836e2_f64) * t43524 * t88794 * t12079 + F::cast_from(0.79025390195226139183e1_f64) * t55732 * t24141 - F::cast_from(0.79025390195226139183e1_f64) * t12122 * t88804 * t3304 + F::cast_from(0.39512695097613069592e1_f64) * t12127 * t88804 * t3318 - F::cast_from(0.79025390195226139183e1_f64) * t16502 * t24135 + F::cast_from(0.15805078039045227836e2_f64) * t19603 * t24090 + F::cast_from(0.52683593463484092788e1_f64) * t4981 * t4893 * t4982 * t23820 + F::cast_from(0.23707617058567841754e2_f64) * t16552 * t19450 * t16553 * t6299 - F::cast_from(0.23707617058567841754e2_f64) * t16559 * t19450 * t16560 * t6299 - F::cast_from(0.15805078039045227836e2_f64) * t12122 * t80264 * t4982 * t1668 + F::cast_from(0.79025390195226139184e1_f64) * t12127 * t80264 * t24083;
    t89603
}
