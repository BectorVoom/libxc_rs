//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3028/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3028<F: Float>(t4930: F, t6305: F, t1024: F, t16449: F, t1651: F, t19453: F, t19521: F, t19526: F, t19556: F, t19573: F, t19576: F, t19603: F, t19608: F, t24123: F, t24126: F, t24147: F, t3299: F, t3304: F, t42261: F, t43384: F, t43598: F, t4772: F, t4964: F, t54695: F, t6258: F, t6362: F, t67595: F, t67644: F, t67652: F) -> (F, F) {
    let t80640 = t4930 * t6305;
    let t80654 = -F::cast_from(0.19756347548806534796e1_f64) * t1024 * t67595 * t1651 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t19556 * t4772 + F::cast_from(0.39512695097613069591e1_f64) * t54695 * t6362 + F::cast_from(0.39512695097613069591e1_f64) * t43598 * t24126 + F::cast_from(0.79025390195226139182e1_f64) * t19526 * t19521 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t16449 * t6258 + F::cast_from(0.65854491829355115987e0_f64) * t43384 * t24123 + F::cast_from(0.39512695097613069591e1_f64) * t3299 * t80640 * t3304 + F::cast_from(0.39512695097613069592e1_f64) * t19603 * t19573 - F::cast_from(0.19756347548806534796e1_f64) * t19608 * t19576 - F::cast_from(0.39512695097613069591e1_f64) * t42261 * t24147 + F::cast_from(0.19756347548806534796e1_f64) * t67644 * t19453 - F::cast_from(0.19756347548806534796e1_f64) * t67652 * t4964;
    (t80640, t80654)
}
