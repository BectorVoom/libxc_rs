//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 765/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk765(t5374: f64, t5391: f64, t592: f64, t1721: f64, t499: f64, t52: f64, t146: f64, t155: f64, t95: f64, t1718: f64, t5292: f64, t5297: f64, t5299: f64, t5301: f64, t5307: f64, t5312: f64, t5315: f64, t5370: f64, t5376: f64, t5379: f64, t5382: f64, t5385: f64, t5390: f64, t590: f64, t612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5393 = t592 * t5374 * t5391;
    let t5397 = t592 * t5374 * t1721;
    let t5401 = 1.0_f64 / t52 / t499;
    let t5402 = t146 * t5401;
    let t5405 = 455.0_f64 / 1296.0_f64 * t5402 * t95 * t155;
    let t5406 = -0.85748036236139473944e-3_f64 * t612 * t5292 - 0.68026775414003982663e-1_f64 * t5297 + 0.12004725073059526352e-1_f64 * t5299 - 0.60023625365297631762e-1_f64 * t5301 - 0.25724410870841842183e-1_f64 * t612 * t5307 + 0.12862205435420921092e-1_f64 * t612 * t5312 + 0.30011812682648815881e-2_f64 * t5315 - 0.21437009059034868486e-3_f64 * t590 * t5370 - 0.21437009059034868486e-3_f64 * t590 * t5376 + 0.30011812682648815881e-2_f64 * t5379 - 0.60023625365297631762e-2_f64 * t5382 - 0.17006693853500995666e-1_f64 * t5385 - 0.12862205435420921092e-2_f64 * t5390 * t5393 + 0.12862205435420921092e-2_f64 * t1718 * t5397 - t5405;
    (t5393, t5397, t5401, t5402, t5405, t5406)
}
