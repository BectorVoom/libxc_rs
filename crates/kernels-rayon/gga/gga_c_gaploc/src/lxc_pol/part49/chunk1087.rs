//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1087/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1087(t42826: f64, t42828: f64, t42829: f64, t42832: f64, t42835: f64, t42838: f64, t42841: f64, t47011: f64, t47013: f64, t47016: f64, t47019: f64, t13729: f64, t6305: f64) -> (f64, f64) {
    let t47023 = 0.94850022118920498663e-2_f64 * t47011 - t42826 + 0.28455006635676149599e-1_f64 * t47013 + 0.28455006635676149599e-1_f64 * t47016 - t47019 + t42828 + 0.56910013271352299198e-1_f64 * t42829 + 0.56910013271352299198e-1_f64 * t42832 + 0.56910013271352299198e-1_f64 * t42835 + t42838 + t42841;
    let t47024 = t6305 * t13729;
    (t47023, t47024)
}
