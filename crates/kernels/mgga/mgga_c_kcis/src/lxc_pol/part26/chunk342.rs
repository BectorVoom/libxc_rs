//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 342/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk342<F: Float>(t1395: F, t2012: F, t1464: F, t1364: F, t1391: F, t1924: F, t1944: F, t1949: F, t1985: F, t2004: F, t2008: F, t507: F) -> (F, F, F) {
    let t2013 = t1395 * t2012;
    let t2014 = t1464 * t2013;
    let t2016 = t1924 * t507 - F::new(0.66725e-1) * t1364 * t1944 + t1391 + F::new(0.16581944444444444444e-2) * t1949 + F::new(0.24872916666666666666e-2) * t1985 - F::new(0.24872916666666666666e-2) * t2004 - F::new(0.66327777777777777776e-2) * t2008 + F::new(0.16581944444444444444e-2) * t2014;
    (t2013, t2014, t2016)
}
