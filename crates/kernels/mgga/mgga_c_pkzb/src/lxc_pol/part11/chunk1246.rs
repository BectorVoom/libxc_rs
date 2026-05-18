//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1246/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1246<F: Float>(t10925: F, t694: F, t10842: F, t10860: F, t10863: F, t10873: F, t10887: F, t1096: F, t17391: F, t17514: F, t17624: F, t1911: F, t21087: F, t21143: F, t21156: F, t26065: F, t2796: F, t2816: F, t2829: F, t30571: F, t30587: F, t3565: F, t3578: F, t3592: F, t3608: F, t5835: F, t695: F, t703: F, t704: F, t7447: F, t9465: F, t9494: F, t9518: F) -> F {
    let t30608 = t10925 * t694;
    let t30617 = F::new(3.0) * t7447 * t3578 + F::new(3.0) * t2796 * t9494 + F::new(1.0) * t695 * (t30571 + t30587) * t703 + F::new(0.2069040516770936012e4) * t17514 * t10863 + F::new(0.17544670867903938621e1) * t2829 * t9465 + F::new(0.51947577317044391276e2) * t21156 * t3608 - F::new(0.10389515463408878255e3) * t17624 * t10887 + F::new(3.0) * t26065 * t1096 + F::new(3.0) * t9518 * t2816 - F::new(0.19298375398431042081e3) * t17391 * t10842 + F::new(1.0) * t1911 * t10860 + F::new(1.0) * t30608 * t704 - F::new(0.35089341735807877242e1) * t21143 * t3592 + F::new(0.35089341735807877242e1) * t5835 * t10873 - F::new(6.0) * t21087 * t3565;
    t30617
}
