//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 794/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk794(t1734: f64, t360: f64, t1089: f64, t368: f64, t372: f64, t1095: f64, t1743: f64, t398: f64, t407: f64, t1795: f64, t301: f64, t3653: f64, t3658: f64, t397: f64, t418: f64, t4946: f64, t4949: f64, t4950: f64, t4953: f64, t4954: f64, t4957: f64, t4961: f64, t4969: f64, t4989: f64, t4994: f64, t4996: f64, t6185: f64, t6195: f64, t6200: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6203 = t1734 * t360;
    let t6205 = t1089 * t368 * t6203;
    let t6209 = t1734 * t372;
    let t6211 = t1089 * t1095 * t6209;
    let t6215 = t398 * t1743 * t407;
    let t6218 = t1795 * t301;
    let t6220 = t1089 * t1095 * t6218;
    let t6223 = -0.42874018118069736972e-3_f64 * t6185 + 0.85748036236139473944e-3_f64 * t3653 - 0.85748036236139473944e-3_f64 * t3658 - 0.17149607247227894789e-2_f64 * t4946 + t4949 + 0.80031500487063509015e-2_f64 * t4950 - t4953 - 0.80031500487063509015e-2_f64 * t4954 - t4957 + t4961 - t4969 - t4989 - 0.42874018118069736972e-3_f64 * t6195 + 0.34299214494455789578e-2_f64 * t4994 + 0.12862205435420921092e-2_f64 * t418 * t6200 - 0.17149607247227894789e-2_f64 * t418 * t6205 - 0.11337795902333997111e-1_f64 * t4996 + 0.17149607247227894789e-2_f64 * t418 * t6211 - 0.42874018118069736972e-3_f64 * t397 * t6215 + 0.17149607247227894789e-2_f64 * t418 * t6220;
    (t6205, t6211, t6215, t6218, t6220, t6223)
}
