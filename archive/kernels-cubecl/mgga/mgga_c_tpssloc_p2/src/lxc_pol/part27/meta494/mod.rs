//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1880;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta494<F: Float>(t23122: F, t25064: F, t4166: F, t6620: F, t849: F, t1516: F, t23127: F, t4261: F, t6621: F, t23133: F, t7503: F, t838: F, t23046: F, t242: F, t812: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25065, t25068, t25069, t25071, t25073, t25077, t25080) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1880::<F>(t23122, t25064, t4166, t6620, t849, t1516, t23127, t4261, t6621, t23133, t7503, t838);
        let (t25083, t25084) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1881::<F>(t23046, t242, t812);
    (t25065, t25068, t25069, t25071, t25073, t25077, t25080, t25083, t25084)
}
