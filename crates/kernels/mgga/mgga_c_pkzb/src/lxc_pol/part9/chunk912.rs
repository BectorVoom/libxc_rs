//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 912/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk912<F: Float>(t2595: F, t6892: F, t168: F, t5389: F, t2591: F, t1034: F, t5391: F, t1719: F, t179: F, t1733: F, t2592: F, t2645: F, t5222: F, t5258: F, t5279: F, t6861: F, t6866: F, t6870: F, t6873: F, t6877: F, t6882: F, t6885: F, t6888: F) -> (F, F, F, F, F, F) {
    let t6894 = F::new(0.40015750243531754508e-2) * t6892 * t2595;
    let t6895 = t5389 * t168;
    let t6896 = t6895 * t2591;
    let t6897 = t1034 * t5391;
    let t6898 = t6897 * t1719;
    let t6899 = t179 * t6898;
    let t6902 = -F::new(7.0) / F::new(48.0) * t5222 - F::new(0.80031500487063509016e-2) * t5258 - F::new(0.21437009059034868486e-3) * t2645 * t6861 + F::new(0.85748036236139473944e-3) * t2592 * t6866 + F::new(0.42874018118069736972e-3) * t2592 * t6870 - F::new(0.80031500487063509015e-2) * t6873 + F::new(0.85748036236139473944e-3) * t1733 * t6877 - F::new(0.21437009059034868486e-3) * t2645 * t6882 - F::new(0.80031500487063509014e-2) * t6885 - F::new(0.42874018118069736972e-2) * t5279 * t6888 - t6894 - F::new(0.12862205435420921092e-2) * t6896 * t6899;
    (t6895, t6896, t6897, t6898, t6899, t6902)
}
