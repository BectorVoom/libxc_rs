//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 912/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk912(t2595: f64, t6892: f64, t168: f64, t5389: f64, t2591: f64, t1034: f64, t5391: f64, t1719: f64, t179: f64, t1733: f64, t2592: f64, t2645: f64, t5222: f64, t5258: f64, t5279: f64, t6861: f64, t6866: f64, t6870: f64, t6873: f64, t6877: f64, t6882: f64, t6885: f64, t6888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6894 = 0.40015750243531754508e-2_f64 * t6892 * t2595;
    let t6895 = t5389 * t168;
    let t6896 = t6895 * t2591;
    let t6897 = t1034 * t5391;
    let t6898 = t6897 * t1719;
    let t6899 = t179 * t6898;
    let t6902 = -7.0_f64 / 48.0_f64 * t5222 - 0.80031500487063509016e-2_f64 * t5258 - 0.21437009059034868486e-3_f64 * t2645 * t6861 + 0.85748036236139473944e-3_f64 * t2592 * t6866 + 0.42874018118069736972e-3_f64 * t2592 * t6870 - 0.80031500487063509015e-2_f64 * t6873 + 0.85748036236139473944e-3_f64 * t1733 * t6877 - 0.21437009059034868486e-3_f64 * t2645 * t6882 - 0.80031500487063509014e-2_f64 * t6885 - 0.42874018118069736972e-2_f64 * t5279 * t6888 - t6894 - 0.12862205435420921092e-2_f64 * t6896 * t6899;
    (t6895, t6896, t6897, t6898, t6899, t6902)
}
