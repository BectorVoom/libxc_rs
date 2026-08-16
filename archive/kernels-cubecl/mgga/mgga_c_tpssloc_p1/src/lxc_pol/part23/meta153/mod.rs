//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk711;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta153<F: Float>(t3441: F, t5392: F, t3440: F, t4904: F, t4919: F, t3455: F, t1177: F, t1178: F, t5398: F, t3464: F, t4770: F, t6012: F, t6015: F, t6018: F, t457: F, t460: F, t974: F, t1714: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6119, t6120, t6123, t6126, t6127, t6130, t6131, t6138) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk711::<F>(t3441, t5392, t3440, t4904, t4919, t3455, t1177, t1178, t5398, t3464, t4770, t6012, t6015, t6018);
        let (t6140, t6141, t6144) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk712::<F>(t457, t6138, t460, t974, t1714);
    (t6119, t6120, t6123, t6126, t6127, t6130, t6131, t6138, t6140, t6141, t6144)
}
