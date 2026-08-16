//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 634/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk634(t1165: f64, t1432: f64, t4267: f64, t3361: f64, t1180: f64, t3477: f64, t3551: f64, t3556: f64, t3562: f64, t397: f64, t418: f64, t4670: f64, t4673: f64, t4675: f64, t4677: f64, t4679: f64, t4689: f64, t4699: f64, t4705: f64, t4716: f64, t4722: f64, t4742: f64, t6071: f64, t6076: f64, t6082: f64, t6086: f64) -> (f64, f64) {
    let t6090 = t1165 * t4267 * t1432;
    let t6091 = t3361 * t6090;
    let t6095 = -0.21437009059034868486e-3_f64 * t3477 - 0.21437009059034868486e-3_f64 * t397 * t6071 - 0.85748036236139473944e-3_f64 * t418 * t6076 + 0.42874018118069736972e-3_f64 * t6082 - t3551 + t3556 - t3562 + 0.80031500487063509015e-2_f64 * t4670 + t4673 - t4675 + t4677 + t4679 - 0.12862205435420921092e-2_f64 * t1180 * t6086 - 0.34299214494455789578e-2_f64 * t6091 + t4689 + t4699 - t4705 - 0.80031500487063509016e-2_f64 * t4716 + t4722 - 0.25724410870841842183e-2_f64 * t4742;
    (t6090, t6095)
}
