//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk959;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk960;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta196<F: Float>(t9898: F, t9994: F, t1390: F, t828: F, t2482: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F, t1410: F, t3934: F, t3944: F, t9932: F, t9937: F, t9944: F, t9953: F, t9958: F, t9963: F, t9966: F, t9971: F, t9973: F, t9977: F, t9982: F, t9986: F, t9993: F, t9755: F, t9824: F, t9928: F, t225: F, t1419: F, t4086: F, t786: F, t4104: F, t268: F, t4056: F, t543: F, t675: F, t4101: F, t555: F, t5744: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9995, t9997, t10001, t10003, t10006) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk959::<F>(t9898, t9994, t1390, t828, t2482, t27, t4000, t221, t4004, t4019, t1410, t3934, t3944, t9932, t9937, t9944, t9953, t9958, t9963, t9966, t9971, t9973, t9977, t9982, t9986, t9993);
        let t10008 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk960::<F>(t10006, t9755, t9824, t9928);
        let (t10009, t10013, t10014, t10015, t10019, t10020, t10022) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk961::<F>(t10008, t225, t1419, t4086, t786, t4104, t268, t4056, t543, t675, t4101, t555, t5744);
    (t9995, t9997, t10001, t10003, t10008, t10009, t10013, t10014, t10015, t10019, t10020, t10022)
}
