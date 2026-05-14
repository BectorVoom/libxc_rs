//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 986/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk986<F: Float>(t14054: F, t3992: F, t2661: F, t5774: F, t72: F, t686: F, t3915: F, t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t2470: F, t5721: F, t1445: F, t5599: F) -> (F, F, F, F, F, F) {
    let t14055 = t3992 * t14054;
    let t14057 = 0.57165357490759649296e-4 * t2661 * t14055;
    let t14078 = t5774 * t72;
    let t14079 = t14078 * t686;
    let t14081 = 0.19514881078765566038e-1 * t3915 * t14079;
    let t14082 = t786 * t5711;
    let t14084 = 0.19514881078765566038e-1 * t14082 * t1364;
    let t14085 = t1357 * t5775;
    let t14087 = 0.10975748638225852664e-1 * t689 * t14085;
    let t14090 = t5721 * t2470;
    let t14091 = t3915 * t14090;
    let t14094 = t5599 * t1445;
    (t14057, t14081, t14084, t14087, t14091, t14094)
}
