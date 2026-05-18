//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 834/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk834<F: Float>(t41726: F, t6717: F, t6914: F, t12878: F, t4953: F, t40073: F, t40076: F, t40090: F, t41596: F, t447: F) -> (F, F, F, F, F, F) {
    let t41729 = F::new(0.37959496694381542179e3) * t6914 * t6717 * t41726;
    let t41734 = F::new(0.62115540045351614476e2) * t4953 * t12878;
    let t41735 = F::new(0.59584149919750711116e-1) * t40073;
    let t41736 = F::new(0.25561950635947166451e0) * t40076;
    let t41737 = F::new(0.19171462976960374838e1) * t40090;
    let t41738 = t41596 * t447;
    (t41729, t41734, t41735, t41736, t41737, t41738)
}
