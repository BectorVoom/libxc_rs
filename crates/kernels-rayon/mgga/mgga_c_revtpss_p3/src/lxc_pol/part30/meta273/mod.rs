//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1206;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1207;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1208;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1209;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1210;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta273(t1294: f64, t2142: f64, t7652: f64, t3140: f64, t487: f64, t1276: f64, t2148: f64, t1243: f64, t1248: f64, t1287: f64, t2150: f64, t473: f64, t7627: f64, t1204: f64, t1215: f64, t1295: f64, t2144: f64, t2149: f64, t2152: f64, t460: f64, t7602: f64, t7629: f64, t7632: f64, t7636: f64, t7639: f64, t7643: f64, t7645: f64, t7648: f64, t7651: f64, t2155: f64, t3801: f64, t33: f64, t265: f64, t502: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t5023: f64, t7193: f64, t2159: f64, t57: f64, t606: f64, t7214: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t7599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7653, t7654, t7658, t7659) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1206(t1294, t2142, t7652, t3140, t487, t1276, t2148);
        let t7660 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1207(t1243, t2142);
        let (t7662, t7666, t7669) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1208(t1248, t1287, t7660, t2150, t473, t7627, t1204, t1215, t1295, t2144, t2149, t2152, t460, t7602, t7629, t7632, t7636, t7639, t7643, t7645, t7648, t7651, t7654, t7659);
        let t7673 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1209(t2155, t3801);
        let (t7677, t7682) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1210(t33, t265, t502, t1298, t1300, t198, t336, t5023, t7193, t7669, t7673, t2159, t57, t606, t7214, dens_threshold, rho1, zeta_threshold);
        let t7683 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1211(t7599, t7682);
    (t7653, t7654, t7658, t7659, t7660, t7662, t7666, t7669, t7673, t7677, t7683)
}
