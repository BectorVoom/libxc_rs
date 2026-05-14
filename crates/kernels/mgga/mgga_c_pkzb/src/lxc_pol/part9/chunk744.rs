//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 744/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk744<F: Float>(t5290: F, t5406: F, t158: F, t625: F, t1791: F, t633: F, t1790: F, t1812: F, t183: F, t5373: F, t1719: F, t621: F, t164: F, t167: F, t1717: F, t1721: F, t1753: F, t1783: F, t2682: F, t2693: F, t5240: F, t5251: F, t5367: F, t5389: F, t5391: F, t588: F, t600: F) -> (F, F, F, F, F, F, F, F) {
    let t5407 = t5290 + t5406;
    let t5408 = t5407 * t158;
    let t5417 = t625 * t625;
    let t5418 = 1.0 / t5417;
    let t5419 = t1791 * t633;
    let t5420 = t5418 * t5419;
    let t5423 = t1790 * t633;
    let t5424 = t5423 * t1812;
    let t5427 = t183 * t5373;
    let t5431 = t621 * t1719;
    let t5462 = -0.39512695097613069591e1 * t5389 * t5427 * t5391 + 0.39512695097613069591e1 * t1717 * t5431 * t1721 + 0.39512695097613069591e1 * t2682 * t5251 + 0.39512695097613069591e1 * t1717 * t5427 * t1721 - 0.19756347548806534796e1 * t588 * t1783 * t600 * t164 - 0.19756347548806534796e1 * t588 * t621 * t1753 * t164 - 0.19756347548806534796e1 * t588 * t5431 * t164 - 0.65854491829355115987e0 * t588 * t183 * t5367 * t164 - 0.19756347548806534796e1 * t2693 * t5240 - 0.65854491829355115987e0 * t588 * t5427 * t164 + 0.65854491829355115987e0 * t167 * t5407;
    (t5407, t5408, t5417, t5418, t5419, t5420, t5424, t5462)
}
