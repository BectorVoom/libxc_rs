//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2570;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2571;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta728<F: Float>(t72: F, t9940: F, t245: F, t543: F, t9400: F, t2713: F, t3964: F, t9714: F, t3951: F, t9732: F, t136: F, t4010: F, t220: F, t1399: F, t3945: F, t9816: F, t13847: F, t4057: F, t9819: F, t9807: F, t9962: F, t9832: F, t2482: F, t27: F, t9991: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47247, t47248, t47249, t47259, t47262, t47273) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2570::<F>(t72, t9940, t245, t543, t9400, t2713, t3964, t9714, t3951, t9732, t136, t4010);
        let (t47274, t47277, t47282, t47284, t47286, t47293) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2571::<F>(t220, t47273, t1399, t3945, t9816, t13847, t4057, t9819, t9807, t9962, t9832, t2482, t27, t9991);
    (t47247, t47248, t47249, t47259, t47262, t47273, t47274, t47277, t47282, t47284, t47286, t47293)
}
