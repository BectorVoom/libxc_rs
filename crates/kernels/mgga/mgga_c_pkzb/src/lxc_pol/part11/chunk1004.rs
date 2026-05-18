//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1004/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1004<F: Float>(t11043: F, t2105: F, t11011: F, t11015: F, t11021: F, t11025: F, t11030: F, t11034: F, t11039: F, t2104: F, t276: F, t2899: F, t2922: F, t9547: F, t9584: F, t9606: F, t9614: F, t9617: F, t9623: F, t9629: F) -> (F, F) {
    let t11044 = t2105 * t11043;
    let t11052 = -t9547 / F::new(96.0) - t276 * t11011 / F::new(96.0) - t276 * t11015 / F::new(16.0) + t9584 / F::new(48.0) + F::new(0.38586616306262763275e-2) * t2104 * t11021 + F::new(0.12862205435420921092e-2) * t2922 * t11025 - F::new(0.64311027177104605458e-3) * t2922 * t11030 - F::new(0.12862205435420921092e-2) * t2104 * t11034 - F::new(0.12862205435420921092e-2) * t2104 * t11039 - F::new(0.25724410870841842183e-2) * t2899 * t11044 - F::new(0.42874018118069736972e-3) * t9606 + F::new(0.85748036236139473944e-3) * t9614 + F::new(0.42874018118069736972e-3) * t9617 - F::new(0.85748036236139473944e-3) * t9623 + F::new(0.25724410870841842184e-2) * t9629;
    (t11044, t11052)
}
