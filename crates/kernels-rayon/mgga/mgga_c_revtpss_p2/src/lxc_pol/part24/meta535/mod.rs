//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1575;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1576;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta535(t22893: f64, t2661: f64, t3992: f64, t48455: f64, t221: f64, t22858: f64, t4019: f64, t47293: f64, t10001: f64, t22863: f64, t22914: f64, t3930: f64, t22865: f64, t9918: f64, t1883: f64, t6883: f64, t9816: f64, t9818: f64, t13999: f64, t22833: f64, t22813: f64, t547: f64, t807: f64, t9941: f64, t1413: f64, t22809: f64, t13767: f64, t1868: f64, t74012: f64, t22953: f64, t543: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86070, t86074, t86078, t86080) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1575(t22893, t2661, t3992, t48455, t221, t22858, t4019, t47293, t10001, t22863, t22914, t3930);
        let (t86112, t86124, t86156, t86165) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1576(t22865, t9918, t1883, t6883, t9816, t9818, t13999, t22833, t22813, t547, t807, t9941);
        let (t86169, t86183, t86203) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1577(t1413, t22809, t547, t807, t13767, t1868, t2661, t74012, t22953, t3992, t543, t550);
    (t86070, t86074, t86078, t86080, t86112, t86124, t86156, t86165, t86169, t86183, t86203)
}
