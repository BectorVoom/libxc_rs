//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 865/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk865<F: Float>(t10144: F, t2343: F, t2268: F, t3327: F, t6313: F, t2317: F, t2761: F, t6525: F, t2321: F, t8237: F, t9074: F, t123: F, t7887: F, t2326: F, t3351: F, t484: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10145 = t2343 * t10144;
    let t10147 = 0.56910013271352299198e-1 * t2268 * t10145;
    let t10150 = 0.37940008847568199465e-1 * t6313 * t3327;
    let t10160 = t2761 * t2317;
    let t10161 = t6525 * t10160;
    let t10162 = 0.11856252764865062333e-2 * t10161;
    let t10163 = t8237 * t2321;
    let t10164 = t9074 * t10163;
    let t10165 = 0.11856252764865062333e-2 * t10164;
    let t10166 = t7887 * t123;
    let t10167 = t10166 * t2326;
    let t10168 = t9074 * t10167;
    let t10169 = 0.35568758294595186999e-2 * t10168;
    let t10175 = t484 * t3351;
    (t10145, t10147, t10150, t10160, t10162, t10163, t10165, t10166, t10167, t10169, t10175)
}
