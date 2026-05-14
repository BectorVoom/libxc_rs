//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1079/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1079<F: Float>(t10673: F, t10676: F, t14668: F, t14675: F, t14678: F, t14682: F, t14690: F, t14693: F, t14697: F, t14703: F, t14705: F, t14707: F, t2745: F, t4362: F, t10815: F, t1561: F) -> (F, F) {
    let t14711 = 0.42874018118069736972e-3 * t4362 * t14668 + t14675 - 0.42874018118069736972e-3 * t2745 * t14678 - 0.21437009059034868486e-3 * t2745 * t14682 - t14690 + 0.17149607247227894789e-2 * t2745 * t14693 + 0.85748036236139473944e-3 * t2745 * t14697 + t14703 + t14705 + 0.17149607247227894789e-2 * t2745 * t14707 + t10673 - 0.14291339372689912324e-3 * t10676;
    let t14712 = t10815 * t1561;
    (t14711, t14712)
}
