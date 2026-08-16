//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3904/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3904(t2782: f64, t4086: f64, t543: f64, t74982: f64, t10130: f64, t1399: f64, t46463: f64, t47995: f64, t47999: f64, t48003: f64, t48005: f64, t48008: f64, t48013: f64, t48020: f64, t49376: f64, t5735: f64, t5745: f64, t5755: f64, t6874: f64, t74965: f64, t74973: f64, t74979: f64, t820: f64) -> f64 {
    let t74985 = t2782 * t4086 * t74982 * t543;
    let t74987 = -0.60712963356159538784e-1_f64 * t46463 - 0.65854491829355115987e0_f64 * t820 * t10130 * t6874 - 0.26341796731742046394e1_f64 * t5755 * t5735 * t49376 + 0.15805078039045227837e2_f64 * t5745 * t5735 * t48020 - 0.13170898365871023197e1_f64 * t5755 * t74965 * t1399 - 0.39029762157531132076e-1_f64 * t47995 - 0.19514881078765566038e-1_f64 * t47999 - 0.46263278077393568556e-2_f64 * t48003 + 0.520396828767081761e-2_f64 * t48005 - 0.13170898365871023197e1_f64 * t5755 * t74973 * t1399 - 0.46263278077393568556e-2_f64 * t48008 + 0.21951497276451705328e-1_f64 * t74979 - 0.19514881078765566038e-1_f64 * t48013 + 0.10975748638225852664e-1_f64 * t74985;
    t74987
}
