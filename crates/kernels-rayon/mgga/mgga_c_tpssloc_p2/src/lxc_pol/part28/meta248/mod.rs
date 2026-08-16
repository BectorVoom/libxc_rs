//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1080;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1081;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1082;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta248(t6889: f64, t6907: f64, t1985: f64, t1887: f64, t534: f64, t6546: f64, t1878: f64, t547: f64, t1329: f64, t1995: f64, t2230: f64, t213: f64, t1999: f64, t533: f64, t556: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6908, t6909, t6914) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1080(t6889, t6907, t1985, t1887, t534, t6546);
        let t6916 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1081(t1878, t547);
        let (t6917, t6919, t6921, t6924) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1082(t1329, t6916, t1995, t2230, t213, t1999, t533, t556);
        let (t6925, t6926) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1083(t598, t6924, t213);
    (t6908, t6909, t6914, t6916, t6917, t6919, t6921, t6924, t6925, t6926)
}
