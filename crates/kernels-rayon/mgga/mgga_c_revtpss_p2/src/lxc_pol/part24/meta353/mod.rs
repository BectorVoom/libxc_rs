//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1217;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta353(t1668: f64, t3154: f64, t19572: f64, t3117: f64, t357: f64, t15696: f64, t6267: f64, t23503: f64, t4915: f64, t11890: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64, t341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1217(t1668, t3154, t19572, t3117, t357, t15696, t6267, t23503, t4915, t11890, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
        let t23959 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1218(t23958, t341);
    (t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958, t23959)
}
