//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta866 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2760;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta866<F: Float>(t22115: F, t9962: F, t13999: F, t22163: F, t22048: F, t22089: F, t22076: F, t6861: F, t9994: F, t1398: F, t125: F, t22252: F, t124: F, t6843: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t73805, t73811, t73813, t73815, t73818, t73820, t73842, t73847) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2760::<F>(t22115, t9962, t13999, t22163, t22048, t22089, t22076, t6861, t9994, t1398, t125, t22252);
        let t73856 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2761::<F>(t124, t6843);
    (t73805, t73811, t73813, t73815, t73818, t73820, t73842, t73847, t73856)
}
