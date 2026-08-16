//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 786/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk786<F: Float>(t1165: F, t1432: F, t4267: F, t3361: F, t1180: F, t3477: F, t3551: F, t3556: F, t3562: F, t397: F, t418: F, t4670: F, t4673: F, t4675: F, t4677: F, t4679: F, t4689: F, t4699: F, t4705: F, t4716: F, t4722: F, t4742: F, t6071: F, t6076: F, t6082: F, t6086: F) -> (F, F) {
    let t6090 = t1165 * t4267 * t1432;
    let t6091 = t3361 * t6090;
    let t6095 = -F::cast_from(0.21437009059034868486e-3_f64) * t3477 - F::cast_from(0.21437009059034868486e-3_f64) * t397 * t6071 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t6076 + F::cast_from(0.42874018118069736972e-3_f64) * t6082 - t3551 + t3556 - t3562 + F::cast_from(0.80031500487063509015e-2_f64) * t4670 + t4673 - t4675 + t4677 + t4679 - F::cast_from(0.12862205435420921092e-2_f64) * t1180 * t6086 - F::cast_from(0.34299214494455789578e-2_f64) * t6091 + t4689 + t4699 - t4705 - F::cast_from(0.80031500487063509016e-2_f64) * t4716 + t4722 - F::cast_from(0.25724410870841842183e-2_f64) * t4742;
    (t6090, t6095)
}
