//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 927/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk927<F: Float>(t161: F, t47008: F, t1358: F, t2339: F, t13735: F, t6305: F, t2268: F, t2440: F, t3691: F, t13751: F, t419: F, t42826: F, t42828: F, t42829: F, t42832: F, t42835: F, t42838: F, t42841: F) -> (F,) {
    let t47009 = t47008 * t161;
    let t47011 = t1358 * t47009 * t2339;
    let t47013 = t6305 * t13735;
    let t47016 = t2268 * t2440 * t3691;
    let t47019 = 0.28455006635676149599e-1 * t419 * t13751;
    let t47023 = 0.94850022118920498663e-2 * t47011 - t42826 + 0.28455006635676149599e-1 * t47013 + 0.28455006635676149599e-1 * t47016 - t47019 + t42828 + 0.56910013271352299198e-1 * t42829 + 0.56910013271352299198e-1 * t42832 + 0.56910013271352299198e-1 * t42835 + t42838 + t42841;
    (t47023,)
}
