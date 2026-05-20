//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3904/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3904<F: Float>(t2782: F, t4086: F, t543: F, t74982: F, t10130: F, t1399: F, t46463: F, t47995: F, t47999: F, t48003: F, t48005: F, t48008: F, t48013: F, t48020: F, t49376: F, t5735: F, t5745: F, t5755: F, t6874: F, t74965: F, t74973: F, t74979: F, t820: F) -> F {
    let t74985 = t2782 * t4086 * t74982 * t543;
    let t74987 = -F::cast_from(0.60712963356159538784e-1_f64) * t46463 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t10130 * t6874 - F::cast_from(0.26341796731742046394e1_f64) * t5755 * t5735 * t49376 + F::cast_from(0.15805078039045227837e2_f64) * t5745 * t5735 * t48020 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t74965 * t1399 - F::cast_from(0.39029762157531132076e-1_f64) * t47995 - F::cast_from(0.19514881078765566038e-1_f64) * t47999 - F::cast_from(0.46263278077393568556e-2_f64) * t48003 + F::cast_from(0.520396828767081761e-2_f64) * t48005 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t74973 * t1399 - F::cast_from(0.46263278077393568556e-2_f64) * t48008 + F::cast_from(0.21951497276451705328e-1_f64) * t74979 - F::cast_from(0.19514881078765566038e-1_f64) * t48013 + F::cast_from(0.10975748638225852664e-1_f64) * t74985;
    t74987
}
