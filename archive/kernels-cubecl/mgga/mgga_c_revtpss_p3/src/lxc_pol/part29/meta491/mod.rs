//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1781;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta491<F: Float>(t233: F, t28340: F, t1957: F, t2061: F, t231: F, t4423: F, t7076: F, t25317: F, t8006: F, t886: F, t4533: F, t7071: F, t27213: F, t7407: F, t1956: F, t26508: F, t26521: F, t26522: F, t26529: F, t26534: F, t26536: F, t26538: F, t27199: F, t4487: F, t7070: F, t7403: F, t7420: F, t2718: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28399, t28400, t28404, t28405, t28411, t28417, t28418) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1780::<F>(t233, t28340, t1957, t2061, t231, t4423, t7076, t25317, t8006, t886, t4533, t7071);
        let t28424 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1781::<F>(t27213, t7407, t1956, t26508, t26521, t26522, t26529, t26534, t26536, t26538, t27199, t28400, t28405, t28411, t28418, t4487, t7070, t7403, t7420);
        let t28425 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1782::<F>(t2061, t2718);
    (t28399, t28400, t28404, t28405, t28411, t28417, t28418, t28424, t28425)
}
