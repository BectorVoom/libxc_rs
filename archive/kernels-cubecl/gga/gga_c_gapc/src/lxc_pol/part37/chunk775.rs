//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 775/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk775<F: Float>(t103: F, t1552: F, t1039: F, t1035: F, t3075: F, t4925: F, t3073: F, t505: F, t674: F, t3143: F, t3139: F, t3060: F, t3120: F) -> (F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t8876 = t103 * t1552;
    let t8877 = t8876 * t1039;
    let t8878 = t1035 * t8877;
    let t8880 = t4925 * t3075;
    let t8881 = t3073 * t8880;
    let t8884 = pi * t505 * t674;
    let t8885 = t8884 * t3143;
    let t8886 = t3139 * t8885;
    let t8888 = t3060 * t3120;
    (t8877, t8878, t8880, t8881, t8884, t8885, t8886, t8888)
}
