//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3318/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3318<F: Float>(t18729: F, t2470: F, t2798: F, t2723: F, t2782: F, t4503: F, t62760: F, t2482: F, t6016: F, t879: F, t2801: F, t14502: F, t18699: F, t2754: F, t40922: F, t40924: F, t4424: F, t4514: F, t51598: F, t51600: F, t51603: F, t51610: F, t51614: F, t51617: F) -> F {
    let t62952 = t2798 * t18729 * t2470;
    let t62961 = t2782 * t4503 * t62760 * t2723;
    let t62967 = t2482 * t879 * t6016;
    let t62968 = t62967 * t2801;
    let t62973 = F::cast_from(0.34146773541147097178e-1_f64) * t40922 - F::cast_from(0.13009920719177044025e-2_f64) * t40924 - F::cast_from(0.39029762157531132076e-1_f64) * t51598 + F::cast_from(0.13009920719177044025e-1_f64) * t62952 - F::cast_from(0.19514881078765566038e-1_f64) * t51600 - F::cast_from(0.46263278077393568556e-2_f64) * t51603 - F::cast_from(0.26341796731742046394e1_f64) * t4514 * t14502 * t4424 - F::cast_from(0.21951497276451705328e-1_f64) * t62961 - F::cast_from(0.21951497276451705328e-1_f64) * t51610 - F::cast_from(0.46263278077393568556e-2_f64) * t51614 - F::cast_from(0.19514881078765566038e-1_f64) * t51617 - F::cast_from(0.19514881078765566038e-1_f64) * t62968 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t18699 * t2754;
    t62973
}
