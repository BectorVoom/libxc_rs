//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1087;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1088;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta244(t5351: f64, t5458: f64, t3766: f64, t487: f64, t460: f64, t3302: f64, t3603: f64, t1248: f64, t5332: f64, t1269: f64, t1287: f64, t1794: f64, t5284: f64, t3781: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5459, t5462, t5463, t5464) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1087(t5351, t5458, t3766, t487, t460, t3302, t3603);
        let t5465 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1088(t1248, t5464);
        let (t5466, t5470, t5474, t5477, t5478, t5479, t5480) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1089(t5332, t5465, t1269, t1287, t1794, t487, t5284, t3781, t460, t1248, t3302, t471);
    (t5459, t5462, t5463, t5464, t5465, t5466, t5470, t5474, t5477, t5478, t5479, t5480)
}
