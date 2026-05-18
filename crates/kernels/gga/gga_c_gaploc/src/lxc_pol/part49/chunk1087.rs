//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1087/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1087<F: Float>(t42826: F, t42828: F, t42829: F, t42832: F, t42835: F, t42838: F, t42841: F, t47011: F, t47013: F, t47016: F, t47019: F, t13729: F, t6305: F) -> (F, F) {
    let t47023 = F::new(0.94850022118920498663e-2) * t47011 - t42826 + F::new(0.28455006635676149599e-1) * t47013 + F::new(0.28455006635676149599e-1) * t47016 - t47019 + t42828 + F::new(0.56910013271352299198e-1) * t42829 + F::new(0.56910013271352299198e-1) * t42832 + F::new(0.56910013271352299198e-1) * t42835 + t42838 + t42841;
    let t47024 = t6305 * t13729;
    (t47023, t47024)
}
