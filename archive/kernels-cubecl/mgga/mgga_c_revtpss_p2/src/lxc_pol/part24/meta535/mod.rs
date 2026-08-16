//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1575;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1576;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta535<F: Float>(t22893: F, t2661: F, t3992: F, t48455: F, t221: F, t22858: F, t4019: F, t47293: F, t10001: F, t22863: F, t22914: F, t3930: F, t22865: F, t9918: F, t1883: F, t6883: F, t9816: F, t9818: F, t13999: F, t22833: F, t22813: F, t547: F, t807: F, t9941: F, t1413: F, t22809: F, t13767: F, t1868: F, t74012: F, t22953: F, t543: F, t550: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t86070, t86074, t86078, t86080) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1575::<F>(t22893, t2661, t3992, t48455, t221, t22858, t4019, t47293, t10001, t22863, t22914, t3930);
        let (t86112, t86124, t86156, t86165) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1576::<F>(t22865, t9918, t1883, t6883, t9816, t9818, t13999, t22833, t22813, t547, t807, t9941);
        let (t86169, t86183, t86203) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1577::<F>(t1413, t22809, t547, t807, t13767, t1868, t2661, t74012, t22953, t3992, t543, t550);
    (t86070, t86074, t86078, t86080, t86112, t86124, t86156, t86165, t86169, t86183, t86203)
}
