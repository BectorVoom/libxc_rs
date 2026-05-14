//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 743/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk743<F: Float>(t5374: F, t5391: F, t592: F, t1721: F, t499: F, t52: F, t146: F, t155: F, t95: F, t1718: F, t5292: F, t5297: F, t5299: F, t5301: F, t5307: F, t5312: F, t5315: F, t5370: F, t5376: F, t5379: F, t5382: F, t5385: F, t5390: F, t590: F, t612: F) -> (F, F, F, F, F, F) {
    let t5393 = t592 * t5374 * t5391;
    let t5397 = t592 * t5374 * t1721;
    let t5401 = 1.0 / t52 / t499;
    let t5402 = t146 * t5401;
    let t5405 = 455.0 / 1296.0 * t5402 * t95 * t155;
    let t5406 = -0.85748036236139473944e-3 * t612 * t5292 - 0.68026775414003982663e-1 * t5297 + 0.12004725073059526352e-1 * t5299 - 0.60023625365297631762e-1 * t5301 - 0.25724410870841842183e-1 * t612 * t5307 + 0.12862205435420921092e-1 * t612 * t5312 + 0.30011812682648815881e-2 * t5315 - 0.21437009059034868486e-3 * t590 * t5370 - 0.21437009059034868486e-3 * t590 * t5376 + 0.30011812682648815881e-2 * t5379 - 0.60023625365297631762e-2 * t5382 - 0.17006693853500995666e-1 * t5385 - 0.12862205435420921092e-2 * t5390 * t5393 + 0.12862205435420921092e-2 * t1718 * t5397 - t5405;
    (t5393, t5397, t5401, t5402, t5405, t5406)
}
