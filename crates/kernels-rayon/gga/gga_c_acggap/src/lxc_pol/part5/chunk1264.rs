//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1264/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1264(t1165: f64, t3391: f64, t4417: f64, t4718: f64, t1095: f64, t1180: f64, t1181: f64, t13949: f64, t13951: f64, t13953: f64, t1426: f64, t1531: f64, t15494: f64, t1552: f64, t17938: f64, t17944: f64, t17946: f64, t17948: f64, t1795: f64, t22401: f64, t418: f64, t4711: f64, t4735: f64, t4762: f64, t5852: f64, t922: f64) -> f64 {
    let t23341 = t3391 * t1165 * t4417 * t4718;
    let t23350 = -0.42874018118069736972e-3_f64 * t1180 * t1181 * t5852 * t4711 - 0.42874018118069736972e-3_f64 * t13949 - 0.85748036236139473945e-2_f64 * t418 * t1426 * t1095 * t1795 * t922 + 0.40015750243531754508e-2_f64 * t13951 + 0.12004725073059526352e-1_f64 * t13953 - 0.68598428988911579156e-2_f64 * t17938 - 0.20579528696673473748e-1_f64 * t4735 * t1165 * t15494 * t4762 - 0.51448821741683684368e-2_f64 * t23341 + 0.12862205435420921092e-2_f64 * t17944 + 0.17149607247227894789e-2_f64 * t17946 - 0.17149607247227894789e-2_f64 * t17948 - 0.51448821741683684367e-2_f64 * t1531 * t1165 * t1552 * t22401;
    t23350
}
