//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 973/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk973<F: Float>(t74693: F, t74695: F, t74698: F, t74701: F, t74705: F, t71033: F, t74662: F, t74667: F, t74670: F, t74677: F, t77143: F, t77148: F, t77150: F, t77154: F, t77155: F, t77156: F, t77157: F) -> F {
    let t77158 = F::cast_from(0.1276937996798935182e-4_f64) * t74693;
    let t77159 = F::cast_from(0.1276937996798935182e-4_f64) * t74695;
    let t77160 = F::cast_from(0.3192344991997337955e-4_f64) * t74698;
    let t77161 = F::cast_from(0.2627895913935205078e-5_f64) * t74701;
    let t77162 = F::cast_from(0.5255791827870410156e-5_f64) * t74705;
    let t77163 = -t71033 + F::cast_from(0.35038612185802734376e-6_f64) * t74662 + F::cast_from(0.8759653046450683594e-6_f64) * t74667 - t74670 - t77143 + F::cast_from(0.76860658247009135557e-5_f64) * t74677 - t77148 + t77150 + t77154 - t77155 + t77156 - t77157 - t77158 + t77159 + t77160 + t77161 - t77162;
    t77163
}
