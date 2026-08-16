//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1155;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1156;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1157;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1158;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta260(t1949: f64, t886: f64, t7071: f64, t822: f64, t867: f64, t231: f64, t836: f64, t233: f64, t7048: f64, t1957: f64, t1956: f64, t1959: f64, t213: f64, t257: f64, t7017: f64, t7020: f64, t7049: f64, t7053: f64, t7062: f64, t7066: f64, t7067: f64, t7070: f64, t887: f64, t892: f64, t1962: f64, t2411: f64, t30: f64, t890: f64, t1940: f64, t1963: f64, t2403: f64, t605: f64, t7010: f64, t1976: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7073, t7076) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1155(t1949, t886, t7071, t822, t867);
        let (t7078, t7079, t7082, t7083, t7086) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1156(t1949, t231, t836, t7076, t233, t7048, t1957, t1956, t1959, t213, t257, t7017, t7020, t7049, t7053, t7062, t7066, t7067, t7070, t7073, t887);
        let t7087 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1157(t7086, t892);
        let t7091 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1158(t1962, t2411);
        let (t7092, t7099, t7102) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1159(t30, t890, t1940, t1963, t2403, t605, t7010, t7087, t7091, t1976, t994);
    (t7073, t7076, t7078, t7079, t7082, t7083, t7086, t7087, t7091, t7092, t7099, t7102)
}
