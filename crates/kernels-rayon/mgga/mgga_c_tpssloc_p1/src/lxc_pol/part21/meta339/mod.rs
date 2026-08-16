//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1725;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1726;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1727;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1728;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta339(t12757: f64, t666: f64, t2358: f64, t4043: f64, t1444: f64, t2342: f64, t9384: f64, t2341: f64, t92: f64, t2219: f64, t659: f64, t2248: f64, t4049: f64, t584: f64, t95: f64, t16: f64, t4053: f64, t1449: f64, t2350: f64, t9398: f64, t100: f64, t2349: f64, t662: f64, t2354: f64, t4059: f64, t103: f64, t4063: f64, t1445: f64, t1447: f64, t2336: f64, t2351: f64, t2355: f64, t4050: f64, t4054: f64, t657: f64, t656: f64, t12747: f64, t12750: f64, t12752: f64, t12754: f64, t64: f64, t9358: f64, t9359: f64, t9361: f64, t9363: f64, t109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12758, t12761, t12771, t12774, t12775, t12778) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1725(t12757, t666, t2358, t4043, t1444, t2342, t9384, t2341, t92, t2219, t659, t2248, t4049);
        let (t12781, t12784, t12792, t12795, t12796, t12799) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1726(t584, t95, t16, t4053, t1449, t2350, t9398, t100, t2349, t2219, t662, t2354, t4059);
        let (t12802, t12805, t12808) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1727(t103, t584, t16, t4063, t100, t12771, t12774, t12775, t12778, t12781, t12784, t12792, t12795, t12796, t12799, t1445, t1447, t2336, t2351, t2355, t4050, t4054, t657, t92);
        let (t12809, t12812) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1728(t12808, t656, t12747, t12750, t12752, t12754, t12758, t12761, t64, t9358, t9359, t9361, t9363);
        let t12813 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1729(t109, t12812);
    (t12758, t12761, t12774, t12792, t12795, t12796, t12799, t12802, t12805, t12808, t12809, t12813)
}
