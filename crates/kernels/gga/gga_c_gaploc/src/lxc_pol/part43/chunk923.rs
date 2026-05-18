//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 923/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk923<F: Float>(t43756: F, t13052: F, t1966: F, t28673: F, t2615: F, t326: F, t43683: F, t8775: F, t9842: F, t41231: F, t41237: F, t41244: F) -> (F, F, F, F, F, F, F, F) {
    let t43757 = F::new(0.19171462976960374838e1) * t43756;
    let t43758 = t1966 * t13052;
    let t43759 = F::new(0.25561950635947166451e1) * t43758;
    let t43760 = t28673 * t13052;
    let t43761 = F::new(0.19171462976960374838e1) * t43760;
    let t43766 = F::new(0.46011511144704899612e1) * t2615 * t326 * t43683;
    let t43774 = F::new(0.11916829983950142223e0) * t8775 * t9842;
    let t43775 = F::new(0.63904876589867916127e-1) * t41231;
    let t43777 = F::new(0.29792074959875355558e-1) * t41237;
    let t43778 = F::new(0.63904876589867916127e-1) * t41244;
    (t43757, t43759, t43761, t43766, t43774, t43775, t43777, t43778)
}
