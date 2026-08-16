//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta404<F: Float>(t13126: F, t487: F, t460: F, t3754: F, t5219: F, t3566: F, t488: F, t1276: F, t1774: F, t1209: F, t1811: F, t1269: F, t1770: F) -> (F, F, F, F, F, F, F) {
        let (t17949, t17958, t17973, t17974, t17986, t17995, t18005) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1451::<F>(t13126, t487, t460, t3754, t5219, t3566, t488, t1276, t1774, t1209, t1811, t1269, t1770);
    (t17949, t17958, t17973, t17974, t17986, t17995, t18005)
}
