//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1291/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1291<F: Float>(t100025: F, t100146: F, t107015: F, t107027: F, t107064: F, t107086: F, t23635: F, t23839: F, t23844: F, t23852: F, t23886: F, t23908: F, t23931: F, t23936: F, t23994: F, t23999: F, t24009: F, t25517: F, t25522: F, t25580: F, t27489: F, t27493: F, t27498: F, t6268: F, t6273: F, t6323: F, t7132: F, t93658: F, t93667: F, t93750: F) -> F {
    let t113634 = F::new(0.25724410870841842183e-2) * t93667 * t23839 - F::new(0.12862205435420921092e-2) * t25580 * t23999 - F::new(0.25724410870841842183e-2) * t100025 * t6273 + F::new(0.17149607247227894789e-2) * t100146 * t6268 - F::new(0.17149607247227894789e-2) * t7132 * t23852 - F::new(0.85748036236139473944e-3) * t107015 + F::new(0.14291339372689912324e-2) * t7132 * t23844 + F::new(0.17149607247227894789e-2) * t107027 + F::new(0.85748036236139473944e-3) * t25517 * t23908 + F::new(0.17149607247227894789e-2) * t25522 * t23635 + F::new(0.17149607247227894789e-2) * t107064 - F::new(0.17149607247227894789e-2) * t107086 + t93750 - F::new(0.12862205435420921092e-2) * t25580 * t23994 + F::new(0.25724410870841842183e-2) * t27493 * t23931 - F::new(0.12862205435420921092e-2) * t27498 * t23936 - F::new(0.25724410870841842183e-2) * t93658 * t24009 - F::new(0.28582678745379824648e-2) * t7132 * t23886 + F::new(0.85748036236139473944e-3) * t27489 * t6323;
    t113634
}
