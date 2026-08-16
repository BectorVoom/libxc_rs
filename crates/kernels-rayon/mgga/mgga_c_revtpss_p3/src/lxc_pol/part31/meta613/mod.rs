//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2055;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta613(t25331: f64, t27216: f64, t212: f64, t27265: f64, t689: f64, t780: f64, t1568: f64, t7063: f64, t25410: f64, t25413: f64, t27299: f64, t93281: f64, t93317: f64, t2439: f64, t7774: f64, t93170: f64, t25304: f64, t27212: f64, t25301: f64, t93371: f64, t27286: f64, t25431: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98825, t98830, t98848, t98849, t98851, t98852, t98853) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2055(t25331, t27216, t212, t27265, t689, t780, t1568, t7063, t25410, t25413, t27299, t93281);
        let (t98856, t98858, t98868, t98875, t98877, t98879) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2056(t93317, t98852, t2439, t7774, t93170, t25304, t27212, t25301, t93371, t27286, t689, t25431);
    (t98825, t98830, t98848, t98849, t98851, t98853, t98856, t98858, t98868, t98875, t98877, t98879)
}
