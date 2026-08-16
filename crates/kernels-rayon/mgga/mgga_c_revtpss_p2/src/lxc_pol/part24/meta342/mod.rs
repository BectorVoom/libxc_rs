//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1193;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta342(t4719: f64, t6219: f64, t15101: f64, t6110: f64, t23466: f64, t935: f64, t2924: f64, t19467: f64, t4711: f64, t981: f64, t1699: f64, t6400: f64, t1079: f64, t1695: f64, t6244: f64, t11133: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23562, t23564, t23565, t23567, t23568, t23570, t23571) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1193(t4719, t6219, t15101, t6110, t23466, t935, t2924, t19467, t4711, t981, t1699, t6400);
        let (t23583, t23598) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1194(t1079, t1695, t6244, t11133, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
    (t23562, t23564, t23565, t23567, t23568, t23570, t23571, t23583, t23598)
}
