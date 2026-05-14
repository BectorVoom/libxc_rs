//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 622/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk622<F: Float>(t2253: F, t2941: F, t3312: F, t3682: F, t4026: F, t4399: F, t1853: F, t979: F, t8418: F, t3255: F, t492: F, t1852: F, t1820: F, t3219: F, t8466: F, t1851: F, t971: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10927 = t2253 * t2941;
    let t10947 = 2.0 * t3312;
    let t10948 = 2.0 * t3682;
    let t10949 = 2.0 * t4026;
    let t10950 = 2.0 * t4399;
    let t10951 = t979 * t1853;
    let t10952 = t8418 * t10951;
    let t10961 = t3255 * t492;
    let t10962 = t1852 * t10961;
    let t10964 = t979 * t1820;
    let t10965 = t1852 * t10964;
    let t10967 = t8466 * t3219;
    let t10969 = t971 * t1851;
    (t10927, t10947, t10948, t10949, t10950, t10952, t10962, t10965, t10967, t10969)
}
