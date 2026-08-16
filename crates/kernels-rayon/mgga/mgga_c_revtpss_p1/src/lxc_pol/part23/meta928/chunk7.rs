//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3028/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3028(t4930: f64, t6305: f64, t1024: f64, t16449: f64, t1651: f64, t19453: f64, t19521: f64, t19526: f64, t19556: f64, t19573: f64, t19576: f64, t19603: f64, t19608: f64, t24123: f64, t24126: f64, t24147: f64, t3299: f64, t3304: f64, t42261: f64, t43384: f64, t43598: f64, t4772: f64, t4964: f64, t54695: f64, t6258: f64, t6362: f64, t67595: f64, t67644: f64, t67652: f64) -> (f64, f64) {
    let t80640 = t4930 * t6305;
    let t80654 = -0.19756347548806534796e1_f64 * t1024 * t67595 * t1651 - 0.19756347548806534796e1_f64 * t1024 * t19556 * t4772 + 0.39512695097613069591e1_f64 * t54695 * t6362 + 0.39512695097613069591e1_f64 * t43598 * t24126 + 0.79025390195226139182e1_f64 * t19526 * t19521 - 0.19756347548806534796e1_f64 * t1024 * t16449 * t6258 + 0.65854491829355115987e0_f64 * t43384 * t24123 + 0.39512695097613069591e1_f64 * t3299 * t80640 * t3304 + 0.39512695097613069592e1_f64 * t19603 * t19573 - 0.19756347548806534796e1_f64 * t19608 * t19576 - 0.39512695097613069591e1_f64 * t42261 * t24147 + 0.19756347548806534796e1_f64 * t67644 * t19453 - 0.19756347548806534796e1_f64 * t67652 * t4964;
    (t80640, t80654)
}
