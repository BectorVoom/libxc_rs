//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1672;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1673;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta356<F: Float>(t12345: F, t1369: F, t12215: F, t12317: F, t12320: F, t12323: F, t12325: F, t12330: F, t12331: F, t12335: F, t12336: F, t12340: F, t3783: F, t3876: F, t559: F, t241: F, t67: F, t6924: F, t12156: F, t820: F, t3866: F, t3872: F, t12012: F, t1367: F, t1339: F, t2690: F, t1336: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12346, t12348) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1672::<F>(t12345, t1369, t12215, t12317, t12320, t12323, t12325, t12330, t12331, t12335, t12336, t12340, t3783, t3876, t559);
        let (t12351, t12353, t12356, t12358, t12361, t12364) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1673::<F>(t241, t67, t6924, t12156, t820, t3866, t3872, t3876, t12012, t1367, t1339, t2690);
        let t12365 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1674::<F>(t12364, t1336);
    (t12346, t12348, t12351, t12353, t12356, t12358, t12361, t12364, t12365)
}
