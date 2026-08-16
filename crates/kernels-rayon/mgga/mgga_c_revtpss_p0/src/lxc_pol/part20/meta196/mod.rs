//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk959;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk960;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta196(t9898: f64, t9994: f64, t1390: f64, t828: f64, t2482: f64, t27: f64, t4000: f64, t221: f64, t4004: f64, t4019: f64, t1410: f64, t3934: f64, t3944: f64, t9932: f64, t9937: f64, t9944: f64, t9953: f64, t9958: f64, t9963: f64, t9966: f64, t9971: f64, t9973: f64, t9977: f64, t9982: f64, t9986: f64, t9993: f64, t9755: f64, t9824: f64, t9928: f64, t225: f64, t1419: f64, t4086: f64, t786: f64, t4104: f64, t268: f64, t4056: f64, t543: f64, t675: f64, t4101: f64, t555: f64, t5744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9995, t9997, t10001, t10003, t10006) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk959(t9898, t9994, t1390, t828, t2482, t27, t4000, t221, t4004, t4019, t1410, t3934, t3944, t9932, t9937, t9944, t9953, t9958, t9963, t9966, t9971, t9973, t9977, t9982, t9986, t9993);
        let t10008 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk960(t10006, t9755, t9824, t9928);
        let (t10009, t10013, t10014, t10015, t10019, t10020, t10022) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk961(t10008, t225, t1419, t4086, t786, t4104, t268, t4056, t543, t675, t4101, t555, t5744);
    (t9995, t9997, t10001, t10003, t10008, t10009, t10013, t10014, t10015, t10019, t10020, t10022)
}
