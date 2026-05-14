//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 962/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk962<F: Float>(t13879: F, t2009: F, t773: F, t38950: F, t955: F, t43904: F, t43908: F, t43909: F, t43910: F, t43911: F, t43913: F, t43915: F, t43918: F, t43919: F, t43922: F, t1445: F, t47187: F, t723: F, t813: F) -> (F, F) {
    let t47430 = 0.35750489951850426669e0 * t773 * t13879 * t2009;
    let t47432 = t955 * t38950;
    let t47436 = -t47430 - 0.25561950635947166451e0 * t43904 + t43908 + 0.23833659967900284446e0 * t47432 - t43909 + t43910 + t43911 - t43913 + t43915 + t43918 - 0.19171462976960374838e0 * t43919 - 0.19171462976960374838e0 * t43922;
    let t47442 = 0.46011511144704899612e1 * t813 * t1445 * t47187 * t723;
    (t47436, t47442)
}
