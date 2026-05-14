//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 937/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk937<F: Float>(t2321: F, t607: F, t4827: F, t4839: F, t4842: F, t4988: F, t4992: F, t4996: F, t5000: F, t5004: F, t5008: F, t5010: F, t5013: F, t5016: F, t5020: F, t5022: F, t6780: F, t6783: F) -> (F, F) {
    let t6798 = t2321 * t607;
    let t6800 = t4988 + t4992 - t6780 + 3.0 * t6798 - t4996 + t5000 + t5004 + t5008 + t5010 + t4827 - t4839 + t5013 - t5016 + t5020 - t4842 - t6783 - t5022;
    (t6798, t6800)
}
