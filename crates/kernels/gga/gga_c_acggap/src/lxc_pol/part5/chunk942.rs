//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 942/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk942<F: Float>(t1265: F, t3892: F, t3930: F, t880: F, t1212: F, t848: F, t1210: F, t310: F, t12357: F, t316: F, t317: F, t2956: F) -> (F, F, F, F, F, F) {
    let t14680 = t3892 * t1265;
    let t14683 = F::new(0.39512695097613069592e1) * t3930 * t880;
    let t14688 = t848 * t1212;
    let t14690 = t310 * t1210;
    let t14695 = F::new(0.65854491829355115987e0) * t316 * t317 * t12357;
    let t14712 = F::new(0.4101607543286562663e4) * t2956;
    (t14680, t14683, t14688, t14690, t14695, t14712)
}
