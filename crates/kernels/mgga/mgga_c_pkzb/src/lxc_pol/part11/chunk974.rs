//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 974/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk974<F: Float>(t10627: F, t51: F, t164: F, t592: F, t1721: F, t10558: F, t10562: F, t10566: F, t10574: F, t10578: F, t10582: F, t10586: F, t10624: F, t1706: F, t1718: F, t1733: F, t2592: F, t2645: F, t5225: F, t580: F, t590: F, t8835: F, t8837: F, t8894: F, t8924: F, t8926: F) -> (F, F, F, F) {
    let t10628 = t51 * t10627;
    let t10630 = t592 * t10628 * t164;
    let t10634 = t592 * t10628 * t1721;
    let t10639 = -t580 * t10558 / F::new(48.0) - t5225 * t10562 / F::new(4.0) + F::new(3.0) / F::new(16.0) * t1706 * t10566 + F::new(0.30011812682648815881e-2) * t8835 - F::new(0.60023625365297631762e-2) * t8837 + F::new(0.30011812682648815881e-2) * t8894 - F::new(0.64311027177104605458e-3) * t2645 * t10574 + F::new(0.12862205435420921092e-2) * t2592 * t10578 + F::new(0.25724410870841842183e-2) * t1733 * t10582 + F::new(0.25724410870841842183e-2) * t1733 * t10586 - F::new(0.21437009059034868486e-3) * t590 * t10624 - F::new(0.21437009059034868486e-3) * t590 * t10630 + F::new(0.12862205435420921092e-2) * t1718 * t10634 - F::new(7.0) / F::new(16.0) * t8924 + F::new(7.0) / F::new(48.0) * t8926;
    (t10628, t10630, t10634, t10639)
}
