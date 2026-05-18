//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 870/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk870<F: Float>(t40612: F, t40614: F, t40620: F, t40630: F, t40632: F, t40634: F, t43072: F, t43073: F, t44855: F, t44857: F, t44858: F, t739: F) -> (F, F) {
    let t44860 = F::new(7.0) / F::new(256.0) * t40612;
    let t44861 = F::new(63.0) / F::new(8192.0) * t40614;
    let t44862 = F::new(63.0) / F::new(524288.0) * t40620;
    let t44863 = F::new(21.0) / F::new(524288.0) * t40630;
    let t44864 = F::new(21.0) / F::new(8192.0) * t40632;
    let t44865 = F::new(7.0) / F::new(768.0) * t40634;
    let t44866 = t44855 - t44857 + t44858 / F::new(2.0) + t43072 - t43073 + t44860 + t44861 - t44862 + t44863 - t44864 - t44865;
    let t44874 = t739 * t44866;
    (t44866, t44874)
}
