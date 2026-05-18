//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 658/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk658<F: Float>(t209: F, t3903: F, t3692: F, t3704: F, t3715: F, t3719: F, t3811: F, t3812: F, t3813: F, t3814: F, t3815: F, t3816: F, t3819: F) -> (F, F) {
    let t3904 = t3903 * t209;
    let t3909 = t3811 - t3812 - t3813 + t3814 - t3815 - t3816 + F::new(0.57970906942607043475e-5) * t3692 - F::new(0.49166375783284505216e-8) * t3704 + t3819 + F::new(0.6629778687778673199e-7) * t3715 - F::new(0.90579542097823505428e-7) * t3719;
    (t3904, t3909)
}
