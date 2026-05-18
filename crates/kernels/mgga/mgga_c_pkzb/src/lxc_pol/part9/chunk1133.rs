//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1133/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1133<F: Float>(t2620: F, t5322: F, t1532: F, t2557: F, t49: F, t4865: F, t7046: F, t4868: F, t1429: F, t1643: F, t1646: F, t439: F) -> (F, F, F, F, F, F) {
    let t19620 = t2620 * t5322;
    let t19621 = F::new(0.56968947174242584612e-3) * t19620;
    let t19623 = t2557 * t49 * t1532;
    let t19624 = F::new(0.32530743900905219526e-1) * t19623;
    let t19625 = t7046 * t4865;
    let t19626 = F::new(0.16265371950452609763e-1) * t19625;
    let t19627 = t7046 * t4868;
    let t19628 = F::new(0.48159733137676571078e0) * t19627;
    let t19633 = t1429 * t1643;
    let t19636 = t439 * t1646;
    (t19621, t19624, t19626, t19628, t19633, t19636)
}
