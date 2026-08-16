//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2416/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2416(t2929: f64, t4446: f64, t1568: f64, t2886: f64, t10737: f64, t13520: f64, t2860: f64, t4408: f64, t10770: f64, t1561: f64, t10524: f64, t10720: f64, t10734: f64, t10743: f64, t10747: f64, t10753: f64, t10756: f64, t10765: f64, t10772: f64, t10805: f64, t14263: f64, t14439: f64, t14443: f64, t14450: f64, t2863: f64, t2906: f64, t2930: f64, t2933: f64, t42020: f64, t42149: f64, t42226: f64, t42228: f64, t4437: f64, t4449: f64, t4454: f64, t4472: f64, t4475: f64) -> (f64, f64) {
    let t49411 = t4446 * t2929;
    let t49422 = t2886 * t1568;
    let t49426 = 6.0_f64 * t13520 * t10737;
    let t49427 = t4408 * t2860;
    let t49430 = t1561 * t10770;
    let t49450 = 0.51947577317044391276e2_f64 * t49411 * t2933 + 0.5848223622634646207e0_f64 * t4449 * t10753 + 0.6233709278045326953e3_f64 * t10756 * t4475 * t10524 + 0.10526802520742363173e2_f64 * t2930 * t4472 * t2906 + 18.0_f64 * t49422 * t10734 - t49426 - 6.0_f64 * t49427 * t2863 - 0.19298375398431042081e3_f64 * t49430 * t10772 + 0.96491876992155210402e2_f64 * t10765 * t14439 + 0.32163958997385070134e2_f64 * t2886 * t4437 * t10805 + 0.6207121550312808036e4_f64 * t42149 * t14443 + 0.19964560303604640732e6_f64 * t42226 * t1568 * t42228 * t10743 - 0.35089341735807877242e1_f64 * t14263 * t10720 - 0.35089341735807877242e1_f64 * t42020 * t4454 - 0.70178683471615754484e1_f64 * t10747 * t14450;
    (t49426, t49450)
}
