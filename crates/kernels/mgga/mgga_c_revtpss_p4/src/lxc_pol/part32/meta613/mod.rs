//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1952;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta613<F: Float>(t18498: F, t27763: F, t106554: F, t27799: F, t18838: F, t33: F, t1353: F, t6922: F, t30105: F, t689: F, t1882: F, t543: F, t5774: F, t1398: F, t6918: F, t1955: F, t27883: F, t1444: F, t6844: F, t1903: F, t5658: F, t1032: F, t6888: F, t1426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t108033, t108036, t108043, t108126, t108138, t108178) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1952::<F>(t18498, t27763, t106554, t27799, t18838, t33, t1353, t6922, t30105, t689, t1882, t543, t5774);
        let (t108206, t108225, t108244, t108259, t108277, t108278) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1953::<F>(t1398, t543, t6918, t1955, t27883, t1444, t6844, t1903, t5658, t1032, t6888, t1426);
    (t108033, t108036, t108043, t108126, t108138, t108178, t108206, t108225, t108244, t108259, t108277, t108278)
}
