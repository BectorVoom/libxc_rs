//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1620;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta419(t372: f64, t6163: f64, t479: f64, t471: f64, t248: f64, t3521: f64, t5979: f64, t1227: f64, t1009: f64, t6150: f64, t1011: f64, t1212: f64, t1226: f64, t6169: f64, t486: f64, t6218: f64, t4978: f64, t4582: f64, t1216: f64, t17635: f64, t4987: f64, t4977: f64, t5012: f64, t11836: f64, t1218: f64, t1232: f64, t15495: f64, t15727: f64, t15731: f64, t15735: f64, t15745: f64, t1737: f64, t3506: f64, t3515: f64, t3536: f64, t4989: f64, t5024: f64, t6221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19031, t19033, t19040, t19041, t19045, t19046, t19047) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1620(t372, t6163, t479, t471, t248, t3521, t5979, t1227, t1009, t6150, t1011, t1212);
        let (t19058, t19062, t19068, t19072, t19075) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1621(t1226, t6169, t486, t6218, t4978, t4582, t1216, t17635, t4987, t4977, t5012, t11836, t1218, t1227, t1232, t15495, t15727, t15731, t15735, t15745, t1737, t19033, t19041, t19047, t3506, t3515, t3536, t4989, t5024, t6221);
    (t19031, t19040, t19045, t19046, t19058, t19062, t19068, t19072, t19075)
}
