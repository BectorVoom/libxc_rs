//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 629/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk629<F: Float>(t12953: F, t9272: F, t12533: F, t12536: F, t12539: F, t12935: F, t12936: F, t12937: F, t12941: F, t12944: F, t12946: F, t12948: F, t12952: F, t3177: F, t993: F, t9263: F) -> (F, F, F) {
    let t12954 = t9272 * t12953;
    let t12955 = 0.11502877786176224903e1 * t12954;
    let t12956 = -t12935 + t12936 + t12937 - t12941 - 0.29792074959875355558e-1 * t12944 + 0.29792074959875355558e-1 * t12946 - t12948 + 0.38342925953920749676e0 * t12533 - 0.38342925953920749676e0 * t12536 - 0.76685851907841499352e0 * t12539 + t12952 - t12955;
    let t12957 = t993 * t3177;
    let t12958 = t9263 * t12957;
    (t12956, t12957, t12958)
}
