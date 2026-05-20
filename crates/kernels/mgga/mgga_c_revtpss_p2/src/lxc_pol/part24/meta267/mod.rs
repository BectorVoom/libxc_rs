//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta267<F: Float>(t3566: F, t488: F, t1276: F, t1774: F, t1209: F, t1828: F, t3736: F, t1811: F, t17306: F, t487: F, t116: F, t5876: F) -> (F, F, F, F, F, F, F) {
        let (t17973, t17974, t17986, t17987, t17995, t18059, t18245) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1039::<F>(t3566, t488, t1276, t1774, t1209, t1828, t3736, t1811, t17306, t487, t116, t5876);
    (t17973, t17974, t17986, t17987, t17995, t18059, t18245)
}
