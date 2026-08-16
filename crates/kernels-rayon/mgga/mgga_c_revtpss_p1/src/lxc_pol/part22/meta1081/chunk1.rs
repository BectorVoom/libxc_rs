//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3895/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3895(t5599: f64, t5775: f64, t689: f64, t10171: f64, t1424: f64, t1445: f64, t22390: f64, t4076: f64, t4131: f64, t4132: f64, t47570: f64, t47574: f64, t47580: f64, t47591: f64, t49497: f64, t49504: f64, t49508: f64, t6918: f64, t6919: f64, t74794: f64, t74797: f64, t74802: f64, t74807: f64, t74810: f64, t74813: f64, t74824: f64, t74826: f64) -> f64 {
    let t74829 = t689 * t5599 * t5775;
    let t74831 = -0.92526556154787137112e-2_f64 * t47570 - 0.39274398764404314548e-3_f64 * t47574 - 0.65854491829355115987e0_f64 * t10171 * t6919 - 0.19514881078765566038e-1_f64 * t74794 + 0.65854491829355115984e-1_f64 * t49497 - 0.39029762157531132074e-1_f64 * t74797 - 0.2601984143835408805e-1_f64 * t47580 - 0.65854491829355115987e0_f64 * t22390 * t4132 - 0.13170898365871023197e1_f64 * t74802 * t1445 - 0.65049603595885220126e-3_f64 * t74807 + 0.10975748638225852664e-1_f64 * t74810 - 0.21951497276451705328e-1_f64 * t74813 - 0.39029762157531132076e-1_f64 * t49504 - t47591 + 0.13170898365871023197e1_f64 * t1424 * t4076 * t6918 * t4131 + 0.21951497276451705328e-1_f64 * t49508 - 0.21951497276451705328e-1_f64 * t74824 + 0.39029762157531132074e-1_f64 * t74826 + 0.21951497276451705328e-1_f64 * t74829;
    t74831
}
