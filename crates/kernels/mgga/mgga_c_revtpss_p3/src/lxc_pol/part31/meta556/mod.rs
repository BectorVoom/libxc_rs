//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1963;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1964;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta556<F: Float>(t25082: F, t30123: F, t7732: F, t7742: F, t1936: F, t6765: F, t651: F, t18245: F, t1501: F, t1518: F, t4248: F, t7741: F, t5920: F, t93: F, t7889: F, t1312: F, t30004: F, t28030: F, t29569: F, t29573: F, t6985: F, t1937: F, t7735: F, t1519: F, t1911: F, t2011: F, t29993: F, t29998: F, t30007: F, t30015: F, t30113: F, t30116: F, t30119: F, t569: F, t5887: F, t5921: F, t6934: F, t7746: F, t7894: F) -> (F, F, F, F, F) {
        let (t30125, t30127, t30128, t30130, t30137, t30138) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1963::<F>(t25082, t30123, t7732, t7742, t1936, t6765, t651, t18245, t1501, t1518);
        let (t30143, t30150) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1964::<F>(t1936, t30138, t4248, t7741, t5920, t93, t7889, t1312, t30004, t1518, t28030, t29569, t29573, t30137, t6985);
        let t30159 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1965::<F>(t18245, t1937, t30138, t4248, t7735, t1519, t1911, t2011, t28030, t29993, t29998, t30007, t30015, t30113, t30116, t30119, t30125, t30127, t30130, t30150, t569, t5887, t5921, t651, t6934, t6985, t7746, t7894);
    (t30128, t30138, t30143, t30150, t30159)
}
