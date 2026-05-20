//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta859 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta859<F: Float>(t1812: F, t3566: F, t1209: F, t13181: F, t1774: F, t17306: F, t1811: F, t21342: F, t21333: F, t487: F, t488: F, t1269: F, t6564: F) -> (F, F, F, F, F, F, F, F) {
        let (t72805, t72808, t72843, t72874, t72877, t72894, t72927, t72933) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2749::<F>(t1812, t3566, t1209, t13181, t1774, t17306, t1811, t21342, t21333, t487, t488, t1269, t6564);
    (t72805, t72808, t72843, t72874, t72877, t72894, t72927, t72933)
}
