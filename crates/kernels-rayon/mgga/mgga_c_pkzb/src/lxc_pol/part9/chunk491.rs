//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 491/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk491(t1987: f64, t732: f64, t1954: f64, t1956: f64, t722: f64, t730: f64, t1971: f64, t713: f64, t1976: f64, t1979: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1989 = 0.11696447245269292414e1_f64 * t1987 * t732;
    let t1991 = t1954 * t1956 * t722;
    let t1993 = 0.11696447245269292414e1_f64 * t730 * t1991;
    let t1995 = t713 * t1971 * t722;
    let t1997 = 0.5848223622634646207e0_f64 * t730 * t1995;
    let t1998 = t1976 * t1956;
    let t1999 = t1998 * t1979;
    let t2001 = 0.17315859105681463759e2_f64 * t730 * t1999;
    let t2002 = t220 * t220;
    let t2003 = 1.0_f64 / t2002;
    (t1989, t1991, t1993, t1995, t1997, t1999, t2001, t2002, t2003)
}
