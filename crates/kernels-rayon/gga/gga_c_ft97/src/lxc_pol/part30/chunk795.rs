//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 795/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk795(t33983: f64, t824: f64, t193: f64, t89: f64, t33953: f64, t799: f64, t27: f64, t33867: f64, t33871: f64, t33876: f64, t33956: f64, t33960: f64, t33964: f64, t33969: f64, t33973: f64, t33977: f64, t33981: f64) -> (f64, f64, f64, f64, f64) {
    let t33984 = t33983 * t824;
    let t33985 = t193 * t33984;
    let t33986 = t89 * t33985;
    let t33988 = t799 * t33953;
    let t33990 = t89 * t27 * t33988;
    let t33992 = t33867 + t33871 / 18.0_f64 + t33876 / 3.0_f64 - t33956 / 6.0_f64 - t33960 - 2.0_f64 / 9.0_f64 * t33964 - 2.0_f64 * t33969 + 4.0_f64 / 3.0_f64 * t33973 + t33977 + t33981 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t33986 - t33990 / 3.0_f64;
    (t33984, t33986, t33988, t33990, t33992)
}
