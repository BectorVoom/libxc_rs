//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1922;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta472(t3154: f64, t4866: f64, t4893: f64, t3117: f64, t11922: f64, t6272: f64, t3115: f64, t1668: f64, t3181: f64, t372: f64, t1045: f64, t4574: f64, t12131: f64, t6266: f64, t15691: f64, t1011: f64, t1068: f64, t15689: f64, t15700: f64, t19951: f64, t19954: f64, t19957: f64, t19960: f64, t19963: f64, t19968: f64, t3106: f64, t4892: f64, t6331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19971, t19972, t19973, t19976, t19977, t19979, t19980, t19981) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1922(t3154, t4866, t4893, t3117, t11922, t6272, t3115, t1668, t3181, t372, t1045, t4574);
        let (t19982, t19985, t19986, t19989) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1923(t19980, t19981, t12131, t6266, t15691, t1011, t1068, t15689, t15700, t19951, t19954, t19957, t19960, t19963, t19968, t19973, t19977, t3106, t4892, t6331);
    (t19971, t19972, t19973, t19976, t19977, t19979, t19981, t19982, t19985, t19986, t19989)
}
