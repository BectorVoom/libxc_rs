//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 975/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk975<F: Float>(t1089: F, t33757: F, t29228: F, t3784: F, t11944: F, t2200: F, t9896: F, t18856: F, t2767: F, t3717: F, t11365: F, t7294: F, t7880: F, t11897: F, t9670: F, t10058: F, t11808: F) -> (F, F, F, F, F, F, F) {
    let t33758 = t33757 * t1089;
    let t33760 = t3784 * t29228;
    let t33763 = t11944 * t2200 * t9896;
    let t33766 = t18856 * t3717 * t2767;
    let t33770 = t7294 * t11365 * t7880;
    let t33772 = t11897 * t9670;
    let t33774 = t11808 * t10058;
    (t33758, t33760, t33763, t33766, t33770, t33772, t33774)
}
