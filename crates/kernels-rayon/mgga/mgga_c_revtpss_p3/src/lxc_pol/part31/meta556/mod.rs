//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1963;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1964;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta556(t25082: f64, t30123: f64, t7732: f64, t7742: f64, t1936: f64, t6765: f64, t651: f64, t18245: f64, t1501: f64, t1518: f64, t4248: f64, t7741: f64, t5920: f64, t93: f64, t7889: f64, t1312: f64, t30004: f64, t28030: f64, t29569: f64, t29573: f64, t6985: f64, t1937: f64, t7735: f64, t1519: f64, t1911: f64, t2011: f64, t29993: f64, t29998: f64, t30007: f64, t30015: f64, t30113: f64, t30116: f64, t30119: f64, t569: f64, t5887: f64, t5921: f64, t6934: f64, t7746: f64, t7894: f64) -> (f64, f64, f64, f64, f64) {
        let (t30125, t30127, t30128, t30130, t30137, t30138) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1963(t25082, t30123, t7732, t7742, t1936, t6765, t651, t18245, t1501, t1518);
        let (t30143, t30150) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1964(t1936, t30138, t4248, t7741, t5920, t93, t7889, t1312, t30004, t1518, t28030, t29569, t29573, t30137, t6985);
        let t30159 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1965(t18245, t1937, t30138, t4248, t7735, t1519, t1911, t2011, t28030, t29993, t29998, t30007, t30015, t30113, t30116, t30119, t30125, t30127, t30130, t30150, t569, t5887, t5921, t651, t6934, t6985, t7746, t7894);
    (t30128, t30138, t30143, t30150, t30159)
}
