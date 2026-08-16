//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta954 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3197;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta954<F: Float>(t1269: F, t13126: F, t460: F, t13147: F, t1770: F, t1204: F, t17852: F, t1209: F, t1284: F, t5412: F, t17845: F, t17306: F, t3754: F, t1774: F, t487: F, t17807: F, t3727: F, t5219: F, t2246: F, t4171: F, t10308: F, t1466: F, t13267: F, t602: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59945, t59948, t59987, t60008, t60013, t60019) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3197::<F>(t1269, t13126, t460, t13147, t1770, t1204, t17852, t1209, t1284, t5412, t17845, t17306, t3754);
        let (t60037, t60087, t60106, t60221, t60224, t60248) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3198::<F>(t1774, t487, t1209, t17807, t3727, t5219, t2246, t4171, t10308, t1466, t13267, t602);
    (t59945, t59948, t59987, t60008, t60013, t60019, t60037, t60087, t60106, t60221, t60224, t60248)
}
