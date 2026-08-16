//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1794;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta506<F: Float>(t6880: F, t7271: F, t6856: F, t6876: F, t7264: F, t26017: F, t6850: F, t26028: F, t6871: F, t6884: F, t7252: F, t25983: F, t6864: F, t1955: F, t6888: F, t1882: F, t1903: F, t543: F, t1868: F, t1907: F, t1501: F, t1518: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t30037, t30039, t30041, t30043, t30045, t30048, t30050) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1794::<F>(t6880, t7271, t6856, t6876, t7264, t26017, t6850, t26028, t6871, t6884, t7252, t25983, t6864);
        let (t30071, t30105, t30122, t30138) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1795::<F>(t1955, t6888, t1882, t1903, t543, t1868, t1907, t1501, t1518);
    (t30037, t30039, t30041, t30043, t30045, t30048, t30050, t30071, t30105, t30122, t30138)
}
