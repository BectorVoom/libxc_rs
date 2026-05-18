//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1264/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1264<F: Float>(t1165: F, t3391: F, t4417: F, t4718: F, t1095: F, t1180: F, t1181: F, t13949: F, t13951: F, t13953: F, t1426: F, t1531: F, t15494: F, t1552: F, t17938: F, t17944: F, t17946: F, t17948: F, t1795: F, t22401: F, t418: F, t4711: F, t4735: F, t4762: F, t5852: F, t922: F) -> F {
    let t23341 = t3391 * t1165 * t4417 * t4718;
    let t23350 = -F::new(0.42874018118069736972e-3) * t1180 * t1181 * t5852 * t4711 - F::new(0.42874018118069736972e-3) * t13949 - F::new(0.85748036236139473945e-2) * t418 * t1426 * t1095 * t1795 * t922 + F::new(0.40015750243531754508e-2) * t13951 + F::new(0.12004725073059526352e-1) * t13953 - F::new(0.68598428988911579156e-2) * t17938 - F::new(0.20579528696673473748e-1) * t4735 * t1165 * t15494 * t4762 - F::new(0.51448821741683684368e-2) * t23341 + F::new(0.12862205435420921092e-2) * t17944 + F::new(0.17149607247227894789e-2) * t17946 - F::new(0.17149607247227894789e-2) * t17948 - F::new(0.51448821741683684367e-2) * t1531 * t1165 * t1552 * t22401;
    t23350
}
