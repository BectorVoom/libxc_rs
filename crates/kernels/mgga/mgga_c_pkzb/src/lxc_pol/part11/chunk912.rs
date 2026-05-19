//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 912/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk912<F: Float>(t6090: F, t6211: F, t7955: F, t8076: F, t9772: F, t9774: F, t9777: F, t9782: F, t9797: F, t9799: F, t9806: F, t9808: F) -> F {
    let t9918 = F::new(0.19419375e1) * t9772 - F::new(0.258925e1) * t9774 - F::new(0.1294625e1) * t9777 + F::new(0.258925e1) * t9799 - t6211 + F::cast_from(0.40256666666666666667e0_f64) * t6090 + F::cast_from(0.80513333333333333333e0_f64) * t7955 - t8076 - F::new(0.301925e0) * t9782 + F::new(0.905775e0) * t9797 - F::cast_from(0.412621875e-1_f64) * t9806 + F::new(0.16504875e0) * t9808;
    t9918
}
