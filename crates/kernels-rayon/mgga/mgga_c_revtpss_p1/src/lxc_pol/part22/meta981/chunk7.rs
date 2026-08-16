//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3318/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3318(t18729: f64, t2470: f64, t2798: f64, t2723: f64, t2782: f64, t4503: f64, t62760: f64, t2482: f64, t6016: f64, t879: f64, t2801: f64, t14502: f64, t18699: f64, t2754: f64, t40922: f64, t40924: f64, t4424: f64, t4514: f64, t51598: f64, t51600: f64, t51603: f64, t51610: f64, t51614: f64, t51617: f64) -> f64 {
    let t62952 = t2798 * t18729 * t2470;
    let t62961 = t2782 * t4503 * t62760 * t2723;
    let t62967 = t2482 * t879 * t6016;
    let t62968 = t62967 * t2801;
    let t62973 = 0.34146773541147097178e-1_f64 * t40922 - 0.13009920719177044025e-2_f64 * t40924 - 0.39029762157531132076e-1_f64 * t51598 + 0.13009920719177044025e-1_f64 * t62952 - 0.19514881078765566038e-1_f64 * t51600 - 0.46263278077393568556e-2_f64 * t51603 - 0.26341796731742046394e1_f64 * t4514 * t14502 * t4424 - 0.21951497276451705328e-1_f64 * t62961 - 0.21951497276451705328e-1_f64 * t51610 - 0.46263278077393568556e-2_f64 * t51614 - 0.19514881078765566038e-1_f64 * t51617 - 0.19514881078765566038e-1_f64 * t62968 - 0.65854491829355115987e0_f64 * t4514 * t18699 * t2754;
    t62973
}
