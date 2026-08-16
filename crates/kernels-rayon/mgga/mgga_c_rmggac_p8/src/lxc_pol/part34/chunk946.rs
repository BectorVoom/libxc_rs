//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 946/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk946(t68418: f64, t73989: f64, t73999: f64, t74003: f64, t15621: f64, t7487: f64, t74013: f64, t74018: f64, t68408: f64, t70799: f64, t73971: f64, t73974: f64, t73977: f64, t73981: f64, t73984: f64, t73994: f64, t74008: f64, t74015: f64) -> f64 {
    let t76808 = 0.60611291211334054834e-6_f64 * t68418;
    let t76814 = 0.11634323970834742769e-4_f64 * t73989;
    let t76816 = 0.23268647941669485538e-4_f64 * t73999;
    let t76817 = 0.85129199786595678799e-5_f64 * t74003;
    let t76819 = t7487 * t15621;
    let t76820 = 0.96056421943322389208e-3_f64 * t76819;
    let t76821 = 0.1276937996798935182e-4_f64 * t74013;
    let t76823 = 0.1276937996798935182e-4_f64 * t74018;
    let t76824 = 0.13469175824740901073e-6_f64 * t68408 + t70799 + t76808 + 0.52557918278704101561e-5_f64 * t73971 - 0.52557918278704101561e-5_f64 * t73974 - 0.17519306092901367187e-5_f64 * t73977 + 0.17519306092901367187e-5_f64 * t73981 - 0.17519306092901367187e-5_f64 * t73984 + t76814 - 0.17451485956252114154e-4_f64 * t73994 + t76816 - t76817 + 0.72714524817717142308e-5_f64 * t74008 + t76820 + t76821 - 0.72714524817717142308e-5_f64 * t74015 - t76823;
    t76824
}
