//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 689/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk689<F: Float>(t2253: F, t2920: F, t2941: F, t3312: F, t3682: F, t4026: F, t4399: F, t1853: F, t979: F, t8418: F, t3255: F, t492: F) -> (F, F, F, F, F, F, F, F) {
    let t10925 = t2253 * t2920;
    let t10927 = t2253 * t2941;
    let t10947 = F::new(2.0) * t3312;
    let t10948 = F::new(2.0) * t3682;
    let t10949 = F::new(2.0) * t4026;
    let t10950 = F::new(2.0) * t4399;
    let t10951 = t979 * t1853;
    let t10952 = t8418 * t10951;
    let t10961 = t3255 * t492;
    (t10925, t10927, t10947, t10948, t10949, t10950, t10952, t10961)
}
