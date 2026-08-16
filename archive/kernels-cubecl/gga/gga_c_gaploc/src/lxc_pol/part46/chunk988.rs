//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 988/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk988<F: Float>(t43758: F, t13052: F, t28673: F, t2676: F, t33139: F, t2615: F, t326: F, t43683: F, t43490: F, t6066: F, t6111: F, t10914: F, t10915: F) -> (F, F, F, F, F, F) {
    let t43759 = F::cast_from(0.25561950635947166451e1_f64) * t43758;
    let t43760 = t28673 * t13052;
    let t43761 = F::cast_from(0.19171462976960374838e1_f64) * t43760;
    let t43762 = t33139 * t2676;
    let t43766 = F::cast_from(0.46011511144704899612e1_f64) * t2615 * t326 * t43683;
    let t43768 = t6111 * t6066 * t43490;
    let t43771 = t10914 * t10915 * t43490;
    (t43759, t43761, t43762, t43766, t43768, t43771)
}
