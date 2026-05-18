//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1377/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1377<F: Float>(t33801: F, t33803: F, t33808: F, t33810: F, t33812: F, t33815: F, t33818: F, t33820: F, t33823: F, t33825: F, t33828: F, t33831: F) -> (F, F) {
    let t36722 = F::new(0.40094868252346065012e-6) * t33801 - F::new(0.21102562238076876322e-7) * t33803 - F::new(0.22098551499687900008e-7) * t33808 - F::new(0.55015711310542948459e-6) * t33810 + F::new(0.40481770833333333336e-4) * t33812 + F::new(0.57920616843011475696e-5) * t33815 - F::new(0.50680539737635041234e-3) * t33818 - F::new(0.34752370105806885418e-3) * t33820 + F::new(0.57920616843011475696e-5) * t33823 - F::new(0.50680539737635041234e-3) * t33825 - F::new(0.34752370105806885418e-3) * t33828;
    let t36723 = F::new(0.69504740211613770836e-3) * t33831;
    (t36722, t36723)
}
