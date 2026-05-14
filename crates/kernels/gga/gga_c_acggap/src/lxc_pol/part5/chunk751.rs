//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 751/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk751<F: Float>(t1743: F, t398: F, t407: F, t1795: F, t301: F, t1089: F, t1095: F, t3653: F, t3658: F, t397: F, t418: F, t4946: F, t4949: F, t4950: F, t4953: F, t4954: F, t4957: F, t4961: F, t4969: F, t4989: F, t4994: F, t4996: F, t6185: F, t6195: F, t6200: F, t6205: F, t6211: F) -> (F, F, F, F) {
    let t6215 = t398 * t1743 * t407;
    let t6218 = t1795 * t301;
    let t6220 = t1089 * t1095 * t6218;
    let t6223 = -0.42874018118069736972e-3 * t6185 + 0.85748036236139473944e-3 * t3653 - 0.85748036236139473944e-3 * t3658 - 0.17149607247227894789e-2 * t4946 + t4949 + 0.80031500487063509015e-2 * t4950 - t4953 - 0.80031500487063509015e-2 * t4954 - t4957 + t4961 - t4969 - t4989 - 0.42874018118069736972e-3 * t6195 + 0.34299214494455789578e-2 * t4994 + 0.12862205435420921092e-2 * t418 * t6200 - 0.17149607247227894789e-2 * t418 * t6205 - 0.11337795902333997111e-1 * t4996 + 0.17149607247227894789e-2 * t418 * t6211 - 0.42874018118069736972e-3 * t397 * t6215 + 0.17149607247227894789e-2 * t418 * t6220;
    (t6215, t6218, t6220, t6223)
}
