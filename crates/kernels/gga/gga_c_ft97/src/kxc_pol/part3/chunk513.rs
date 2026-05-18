//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 513/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk513<F: Float>(t3977: F, t766: F, t242: F, t1168: F, t2469: F, t1170: F, t1882: F, t1144: F, t1175: F, t713: F, t729: F, t1131: F, t773: F) -> (F, F, F, F, F, F, F, F) {
    let t3978 = t3977 * t766;
    let t3979 = t242 * t3978;
    let t3982 = t2469 * t1168;
    let t3983 = t242 * t3982;
    let t3986 = t1882 * t1170;
    let t3988 = t1882 * t1144;
    let t3991 = t729 * t1175 * t713;
    let t3995 = t729 * t773 * t1131;
    (t3978, t3979, t3982, t3983, t3986, t3988, t3991, t3995)
}
