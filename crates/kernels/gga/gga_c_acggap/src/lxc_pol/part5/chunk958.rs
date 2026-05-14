//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 958/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk958<F: Float>(t12746: F, t1530: F, t1535: F, t12743: F, t1562: F, t3431: F, t4410: F, t14056: F, t4269: F, t1111: F, t13299: F, t17173: F, t406: F, t8790: F, t14176: F, t5286: F) -> (F, F, F, F, F, F) {
    let t18768 = t1530 * t12746 * t1535;
    let t18770 = t12743 * t1562;
    let t18772 = t3431 * t4410;
    let t18788 = t14056 * t4269;
    let t18805 = t17173 * t13299 * t8790 * t1111 * t406;
    let t18808 = t14176 * t5286;
    (t18768, t18770, t18772, t18788, t18805, t18808)
}
