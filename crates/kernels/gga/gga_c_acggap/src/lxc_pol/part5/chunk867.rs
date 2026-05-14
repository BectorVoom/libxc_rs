//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 867/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk867<F: Float>(t3101: F, t316: F, t449: F, t463: F, t1265: F, t3892: F, t3930: F, t880: F, t1212: F, t848: F, t1210: F, t310: F, t12357: F, t317: F, t2956: F, t2709: F) -> (F, F, F, F, F, F, F, F) {
    let t14678 = t316 * t449 * t3101 * t463;
    let t14680 = t3892 * t1265;
    let t14683 = 0.39512695097613069592e1 * t3930 * t880;
    let t14688 = t848 * t1212;
    let t14690 = t310 * t1210;
    let t14695 = 0.65854491829355115987e0 * t316 * t317 * t12357;
    let t14712 = 0.4101607543286562663e4 * t2956;
    let t14717 = 0.28493333333333333333e0 * t2709;
    (t14678, t14680, t14683, t14688, t14690, t14695, t14712, t14717)
}
