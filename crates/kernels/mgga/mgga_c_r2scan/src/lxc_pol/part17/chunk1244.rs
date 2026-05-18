//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1244/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1244<F: Float>(t38649: F, t40228: F, t41751: F, t41756: F, t41757: F, t43654: F, t43657: F, t43660: F, t43664: F, t43667: F, t43670: F, t43672: F) -> F {
    let t44492 = -F::new(0.13170898365871023197e1) * t43654 + F::new(0.65854491829355115984e0) * t43657 - t41751 - F::new(0.46574606203128791246e-1) * t43660 + F::new(0.65049603595885220124e-3) * t40228 + t41756 + t41757 + F::new(0.87327386630866483588e-2) * t43664 - F::new(0.43663693315433241794e-2) * t43667 - F::new(0.13099107994629972538e-1) * t43670 - t38649 - F::new(0.17336443480108537126e0) * t43672;
    t44492
}
