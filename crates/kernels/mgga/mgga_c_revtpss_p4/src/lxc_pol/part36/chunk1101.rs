//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1101/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1101<F: Float>(t24973: F, t3783: F, t1794: F, t3302: F, t471: F, t20800: F, t24834: F, t3769: F, t1287: F, t24770: F, t487: F, t1234: F, t12717: F, t12751: F, t12756: F, t1285: F, t17183: F, t17192: F, t17307: F, t1770: F, t17958: F, t24698: F, t24964: F, t24974: F, t24978: F, t24981: F, t24986: F, t24989: F, t3755: F, t3767: F, t3782: F, t490: F, t5326: F, t5463: F, t5478: F, t6714: F, t6717: F, t6723: F, t6738: F, t6741: F) -> (F, F) {
    let t24994 = t24973 * t3783;
    let t24998 = t3302 * t1794 * t471;
    let t24999 = t20800 * t24998;
    let t25002 = t24834 * t3769;
    let t25005 = t24834 * t3783;
    let t25009 = t487 * t24770 * t1287;
    let t25014 = F::cast_from(0.65854491829355115987e0_f64) * t24698 * t490 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t24964 + F::cast_from(0.39512695097613069591e1_f64) * t17307 * t6714 + F::cast_from(0.19756347548806534796e1_f64) * t1770 * t6741 - F::cast_from(0.19756347548806534796e1_f64) * t17183 * t6738 + F::cast_from(0.39512695097613069591e1_f64) * t3767 * t24974 + F::cast_from(0.39512695097613069591e1_f64) * t5463 * t24978 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t24981 - F::cast_from(0.39512695097613069591e1_f64) * t17192 * t6717 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t24986 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t24989 - F::cast_from(0.39512695097613069591e1_f64) * t17958 * t6717 - F::cast_from(0.19756347548806534796e1_f64) * t3782 * t24994 - F::cast_from(0.19756347548806534796e1_f64) * t5478 * t24999 - F::cast_from(0.39512695097613069591e1_f64) * t12751 * t25002 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t25005 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t25009 - F::cast_from(0.19756347548806534796e1_f64) * t5326 * t6723;
    (t24998, t25014)
}
