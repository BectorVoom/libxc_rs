//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3895/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3895<F: Float>(t5599: F, t5775: F, t689: F, t10171: F, t1424: F, t1445: F, t22390: F, t4076: F, t4131: F, t4132: F, t47570: F, t47574: F, t47580: F, t47591: F, t49497: F, t49504: F, t49508: F, t6918: F, t6919: F, t74794: F, t74797: F, t74802: F, t74807: F, t74810: F, t74813: F, t74824: F, t74826: F) -> F {
    let t74829 = t689 * t5599 * t5775;
    let t74831 = -F::cast_from(0.92526556154787137112e-2_f64) * t47570 - F::cast_from(0.39274398764404314548e-3_f64) * t47574 - F::cast_from(0.65854491829355115987e0_f64) * t10171 * t6919 - F::cast_from(0.19514881078765566038e-1_f64) * t74794 + F::cast_from(0.65854491829355115984e-1_f64) * t49497 - F::cast_from(0.39029762157531132074e-1_f64) * t74797 - F::cast_from(0.2601984143835408805e-1_f64) * t47580 - F::cast_from(0.65854491829355115987e0_f64) * t22390 * t4132 - F::cast_from(0.13170898365871023197e1_f64) * t74802 * t1445 - F::cast_from(0.65049603595885220126e-3_f64) * t74807 + F::cast_from(0.10975748638225852664e-1_f64) * t74810 - F::cast_from(0.21951497276451705328e-1_f64) * t74813 - F::cast_from(0.39029762157531132076e-1_f64) * t49504 - t47591 + F::cast_from(0.13170898365871023197e1_f64) * t1424 * t4076 * t6918 * t4131 + F::cast_from(0.21951497276451705328e-1_f64) * t49508 - F::cast_from(0.21951497276451705328e-1_f64) * t74824 + F::cast_from(0.39029762157531132074e-1_f64) * t74826 + F::cast_from(0.21951497276451705328e-1_f64) * t74829;
    t74831
}
