//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1664;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1665;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta442<F: Float>(t3379: F, t5105: F, t12327: F, t1723: F, t3391: F, t12331: F, t3390: F, t5079: F, t1134: F, t3399: F, t5071: F, t3407: F, t5087: F, t5101: F, t698: F, t1145: F, t16746: F, t141: F, t16712: F, t1729: F, t2439: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16742: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16846, t16852, t16855, t16858, t16860, t16862) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1664::<F>(t3379, t5105, t12327, t1723, t3391, t12331, t3390, t5079, t1134, t3399, t5071, t3407);
        let (t16863, t16865, t16868, t16869, t16871, t16873, t16876) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1665::<F>(t1134, t16862, t3399, t5087, t5101, t698, t1145, t16746, t141, t16712, t1729, t2439);
        let (t16883, t16886) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1666::<F>(t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16869, t16871, t16873, t16876, t1145, t16742);
    (t16846, t16852, t16855, t16858, t16860, t16863, t16865, t16868, t16871, t16876, t16883, t16886)
}
