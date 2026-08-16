//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1952;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta613(t18498: f64, t27763: f64, t106554: f64, t27799: f64, t18838: f64, t33: f64, t1353: f64, t6922: f64, t30105: f64, t689: f64, t1882: f64, t543: f64, t5774: f64, t1398: f64, t6918: f64, t1955: f64, t27883: f64, t1444: f64, t6844: f64, t1903: f64, t5658: f64, t1032: f64, t6888: f64, t1426: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108033, t108036, t108043, t108126, t108138, t108178) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1952(t18498, t27763, t106554, t27799, t18838, t33, t1353, t6922, t30105, t689, t1882, t543, t5774);
        let (t108206, t108225, t108244, t108259, t108277, t108278) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1953(t1398, t543, t6918, t1955, t27883, t1444, t6844, t1903, t5658, t1032, t6888, t1426);
    (t108033, t108036, t108043, t108126, t108138, t108178, t108206, t108225, t108244, t108259, t108277, t108278)
}
