//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1037/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1037<F: Float>(t43756: F, t13052: F, t1966: F, t28673: F, t2676: F, t33139: F, t2615: F, t326: F, t43683: F, t43490: F, t6066: F, t6111: F) -> (F, F, F, F, F, F) {
    let t43757 = F::new(0.19171462976960374838e1) * t43756;
    let t43758 = t1966 * t13052;
    let t43759 = F::new(0.25561950635947166451e1) * t43758;
    let t43760 = t28673 * t13052;
    let t43761 = F::new(0.19171462976960374838e1) * t43760;
    let t43762 = t33139 * t2676;
    let t43766 = F::new(0.46011511144704899612e1) * t2615 * t326 * t43683;
    let t43768 = t6111 * t6066 * t43490;
    (t43757, t43759, t43761, t43762, t43766, t43768)
}
