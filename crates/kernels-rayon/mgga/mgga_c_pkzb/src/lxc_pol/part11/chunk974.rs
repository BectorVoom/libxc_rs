//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 974/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk974(t10627: f64, t51: f64, t164: f64, t592: f64, t1721: f64, t10558: f64, t10562: f64, t10566: f64, t10574: f64, t10578: f64, t10582: f64, t10586: f64, t10624: f64, t1706: f64, t1718: f64, t1733: f64, t2592: f64, t2645: f64, t5225: f64, t580: f64, t590: f64, t8835: f64, t8837: f64, t8894: f64, t8924: f64, t8926: f64) -> (f64, f64, f64, f64) {
    let t10628 = t51 * t10627;
    let t10630 = t592 * t10628 * t164;
    let t10634 = t592 * t10628 * t1721;
    let t10639 = -t580 * t10558 / 48.0_f64 - t5225 * t10562 / 4.0_f64 + 3.0_f64 / 16.0_f64 * t1706 * t10566 + 0.30011812682648815881e-2_f64 * t8835 - 0.60023625365297631762e-2_f64 * t8837 + 0.30011812682648815881e-2_f64 * t8894 - 0.64311027177104605458e-3_f64 * t2645 * t10574 + 0.12862205435420921092e-2_f64 * t2592 * t10578 + 0.25724410870841842183e-2_f64 * t1733 * t10582 + 0.25724410870841842183e-2_f64 * t1733 * t10586 - 0.21437009059034868486e-3_f64 * t590 * t10624 - 0.21437009059034868486e-3_f64 * t590 * t10630 + 0.12862205435420921092e-2_f64 * t1718 * t10634 - 7.0_f64 / 16.0_f64 * t8924 + 7.0_f64 / 48.0_f64 * t8926;
    (t10628, t10630, t10634, t10639)
}
