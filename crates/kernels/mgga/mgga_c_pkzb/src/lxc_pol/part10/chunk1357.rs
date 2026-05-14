//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1357/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1357<F: Float>(t10097: F, t3185: F, t926: F, t10083: F, t10251: F, t2185: F, t22972: F, t23061: F, t23081: F, t27020: F, t27062: F, t27073: F, t27076: F, t27083: F, t27085: F, t27104: F, t27113: F, t27119: F, t3206: F, t3207: F, t3235: F, t406: F, t758: F, t824: F, t8260: F, t8319: F, t8377: F, t8430: F, t8436: F, t8445: F, t8450: F, t8451: F) -> (F,) {
    let t27122 = t3185 * t926 * t10097;
    let t27124 = 0.19055119163586549765e-3 * t27073 - 0.28582678745379824648e-3 * t27076 - 0.17149607247227894789e-2 * t3185 * t27020 * t8260 + 0.17149607247227894789e-2 * t23061 + 0.17149607247227894789e-2 * t27083 + 0.25724410870841842184e-2 * t3235 * t758 * t27085 * t824 + 0.12862205435420921092e-2 * t3235 * t758 * t10251 * t2185 - 0.77173232612525526552e-2 * t23081 * t406 * t27062 * t8430 + 0.30011812682648815881e-2 * t22972 * t406 * t27062 * t8436 - 0.13719685797782315831e-1 * t8319 * t8377 - 0.42874018118069736972e-3 * t3206 * t406 * t27104 * t3207 - 0.21437009059034868486e-3 * t3206 * t406 * t10083 * t8445 + 0.21437009059034868486e-3 * t8450 * t406 * t27113 * t8451 + 0.17149607247227894789e-2 * t27119 + 0.57165357490759649296e-3 * t27122;
    (t27124,)
}
