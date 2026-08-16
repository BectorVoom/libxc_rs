//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2575/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2575(t1129: f64, t1137: f64, t15121: f64, t15141: f64, t1695: f64, t18644: f64, t18840: f64, t18894: f64, t18899: f64, t21855: f64, t21887: f64, t21890: f64, t3327: f64, t436: f64, t44172: f64, t44214: f64, t4797: f64, t4820: f64, t4858: f64, t51392: f64, t51599: f64, t6053: f64, t6056: f64, t6085: f64, t63597: f64, t71876: f64, t71879: f64, t71902: f64, t71915: f64, t71929: f64, t71941: f64, t71955: f64, t71968: f64, t71978: f64, t71989: f64, t72019: f64, t72037: f64) -> f64 {
    let t72041 = 0.17544670867903938621e1_f64 * t63597 * t1695 + 0.17544670867903938621e1_f64 * t18899 * t4858 + 0.17544670867903938621e1_f64 * t15121 * t6085 + t71876 - t71879 + 3.0_f64 * t18840 * t4820 + 3.0_f64 * t15141 * t6053 + 3.0_f64 * t4797 * t18894 + 0.96491876992155210402e2_f64 * t51599 * t6056 - 0.19298375398431042081e3_f64 * t44214 * t21855 + 1.0_f64 * t3327 * t21887 + 1.0_f64 * t1129 * (t71902 + t71915 + t71929 + t71941 + t71955 + t71968 + t71978 + t71989) * t1137 + 0.2069040516770936012e4_f64 * t44172 * t21890 - 0.57895126195293126241e3_f64 * t51392 * t18644 - 0.310907e-1_f64 * (t72019 + t72037) * t436;
    t72041
}
