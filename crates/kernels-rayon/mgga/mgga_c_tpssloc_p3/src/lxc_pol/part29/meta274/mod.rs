//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1275;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1276;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1277;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta274(t1799: f64, t6968: f64, t6637: f64, t6888: f64, t5335: f64, t550: f64, t6976: f64, t1992: f64, t1834: f64, t1998: f64, t214: f64, t1985: f64, t1825: f64, t6987: f64, t553: f64, t7722: f64, t1336: f64, t1814: f64, t2013: f64, t544: f64, t6967: f64, t6975: f64, t1378: f64, t1375: f64, t1843: f64, t2016: f64, t5215: f64, t5321: f64, t568: f64, t6885: f64, t6900: f64, t6958: f64, t7693: f64, t7698: f64, t7702: f64, t7704: f64, t7723: f64, t7729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1275(t1799, t6968, t6637, t6888, t5335, t550, t6976, t1992, t1834, t1998, t214, t1985);
        let (t7745, t7747, t7749) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1276(t1825, t6987, t553, t7722, t1336, t1814, t2013, t544, t6967, t6975, t7734, t7738, t7742);
        let t7750 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1277(t1378, t7749);
        let t7752 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1278(t1375, t1843, t2016, t5215, t5321, t568, t6885, t6900, t6958, t7693, t7698, t7702, t7704, t7723, t7729, t7750);
    (t7732, t7733, t7736, t7737, t7740, t7741, t7745, t7747, t7749, t7750, t7752)
}
