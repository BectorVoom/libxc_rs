//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 983/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk983<F: Float>(t35087: F, t8392: F, t23478: F, t6718: F, t23997: F, t26523: F, t2179: F, t34947: F, t609: F, t1882: F, t35122: F, t35203: F, t35214: F, t139573: F, t3483: F, t35208: F) -> (F, F, F, F, F, F, F, F, F) {
    let t148194 = t8392 * t35087;
    let t148196 = t23478 * t6718;
    let t148205 = t23997 * t26523;
    let t148210 = t2179 * t34947 * t609;
    let t148219 = t1882 * t35122;
    let t148221 = t1882 * t35203;
    let t148223 = t1882 * t35214;
    let t148225 = t139573 * t3483;
    let t148229 = t1882 * t35208;
    (t148194, t148196, t148205, t148210, t148219, t148221, t148223, t148225, t148229)
}
