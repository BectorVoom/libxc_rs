//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 690/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk690<F: Float>(t10961: F, t1852: F, t1820: F, t979: F, t3219: F, t8466: F, t1851: F, t971: F, t1853: F, t1904: F, t2983: F, t7793: F) -> (F, F, F, F, F, F) {
    let t10962 = t1852 * t10961;
    let t10964 = t979 * t1820;
    let t10965 = t1852 * t10964;
    let t10967 = t8466 * t3219;
    let t10969 = t971 * t1851;
    let t10970 = t10969 * t1853;
    let t10974 = t2983 * t1904;
    let t10975 = t7793 * t10974;
    (t10962, t10965, t10967, t10970, t10974, t10975)
}
