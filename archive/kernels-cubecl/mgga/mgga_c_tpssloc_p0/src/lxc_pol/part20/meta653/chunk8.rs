//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2416/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2416<F: Float>(t2929: F, t4446: F, t1568: F, t2886: F, t10737: F, t13520: F, t2860: F, t4408: F, t10770: F, t1561: F, t10524: F, t10720: F, t10734: F, t10743: F, t10747: F, t10753: F, t10756: F, t10765: F, t10772: F, t10805: F, t14263: F, t14439: F, t14443: F, t14450: F, t2863: F, t2906: F, t2930: F, t2933: F, t42020: F, t42149: F, t42226: F, t42228: F, t4437: F, t4449: F, t4454: F, t4472: F, t4475: F) -> (F, F) {
    let t49411 = t4446 * t2929;
    let t49422 = t2886 * t1568;
    let t49426 = F::cast_from(6.0_f64) * t13520 * t10737;
    let t49427 = t4408 * t2860;
    let t49430 = t1561 * t10770;
    let t49450 = F::cast_from(0.51947577317044391276e2_f64) * t49411 * t2933 + F::cast_from(0.5848223622634646207e0_f64) * t4449 * t10753 + F::cast_from(0.6233709278045326953e3_f64) * t10756 * t4475 * t10524 + F::cast_from(0.10526802520742363173e2_f64) * t2930 * t4472 * t2906 + F::cast_from(18.0_f64) * t49422 * t10734 - t49426 - F::cast_from(6.0_f64) * t49427 * t2863 - F::cast_from(0.19298375398431042081e3_f64) * t49430 * t10772 + F::cast_from(0.96491876992155210402e2_f64) * t10765 * t14439 + F::cast_from(0.32163958997385070134e2_f64) * t2886 * t4437 * t10805 + F::cast_from(0.6207121550312808036e4_f64) * t42149 * t14443 + F::cast_from(0.19964560303604640732e6_f64) * t42226 * t1568 * t42228 * t10743 - F::cast_from(0.35089341735807877242e1_f64) * t14263 * t10720 - F::cast_from(0.35089341735807877242e1_f64) * t42020 * t4454 - F::cast_from(0.70178683471615754484e1_f64) * t10747 * t14450;
    (t49426, t49450)
}
