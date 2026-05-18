//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 491/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk491<F: Float>(t1987: F, t732: F, t1954: F, t1956: F, t722: F, t730: F, t1971: F, t713: F, t1976: F, t1979: F, t220: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1989 = F::new(0.11696447245269292414e1) * t1987 * t732;
    let t1991 = t1954 * t1956 * t722;
    let t1993 = F::new(0.11696447245269292414e1) * t730 * t1991;
    let t1995 = t713 * t1971 * t722;
    let t1997 = F::new(0.5848223622634646207e0) * t730 * t1995;
    let t1998 = t1976 * t1956;
    let t1999 = t1998 * t1979;
    let t2001 = F::new(0.17315859105681463759e2) * t730 * t1999;
    let t2002 = t220 * t220;
    let t2003 = F::new(1.0) / t2002;
    (t1989, t1991, t1993, t1995, t1997, t1999, t2001, t2002, t2003)
}
