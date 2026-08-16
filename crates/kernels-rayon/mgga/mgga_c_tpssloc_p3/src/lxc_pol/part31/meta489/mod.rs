//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1669;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1670;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1671;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta489(t1860: f64, t26959: f64, t2032: f64, t22549: f64, t23963: f64, t23968: f64, t23970: f64, t23973: f64, t23978: f64, t23995: f64, t23999: f64, t26009: f64, t26016: f64, t26028: f64, t26945: f64, t26948: f64, t26954: f64, t6486: f64, t7035: f64, t7428: f64, t7782: f64, t5: f64, t26938: f64, t112: f64, t24990: f64, t7170: f64, t24432: f64, t25988: f64, t2035: f64, t671: f64, t1393: f64, t1459: f64, t1849: f64, t1983: f64, t2040: f64, t2079: f64, t22574: f64, t26114: f64, t26898: f64, t26902: f64, t26906: f64, t4037: f64, t510: f64, t5361: f64, t650: f64, t6876: f64, t7042: f64, t7166: f64, t7218: f64, t7685: f64, t7890: f64, t7900: f64, t7941: f64, t26198: f64, t12020: f64, t2091: f64, t5325: f64, t26200: f64, t3887: f64, t5353: f64, t1375: f64, t26184: f64, t26187: f64, t26191: f64, t26195: f64, t26204: f64, t26207: f64, t26212: f64, t26224: f64, t3758: f64, t5326: f64, t7194: f64, t7925: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26960, t26964) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1669(t1860, t26959, t2032, t22549, t23963, t23968, t23970, t23973, t23978, t23995, t23999, t26009, t26016, t26028, t26945, t26948, t26954, t6486, t7035, t7428, t7782);
        let (t26966, t26967, t26969, t26974, t26977) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1670(t5, t26938, t26964, t112, t24990, t7170, t24432, t25988, t2035, t671);
        let t26982 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1671(t1393, t1459, t1849, t1983, t2040, t2079, t22574, t26114, t26898, t26902, t26906, t26967, t26969, t26974, t26977, t4037, t510, t5361, t650, t6876, t7042, t7166, t7218, t7685, t7890, t7900, t7941);
        let (t26989, t26990, t26996, t27005) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1672(t26198, t12020, t2091, t5325, t26200, t3887, t5353, t1375, t26184, t26187, t26191, t26195, t26204, t26207, t26212, t26224, t3758, t5326, t7194, t7925);
    (t26960, t26966, t26967, t26969, t26974, t26977, t26982, t26989, t26990, t26996, t27005)
}
