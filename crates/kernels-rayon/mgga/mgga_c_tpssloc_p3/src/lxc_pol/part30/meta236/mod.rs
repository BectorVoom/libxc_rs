//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1064;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1065;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta236(t6168: f64, t68: f64, t484: f64, t3560: f64, t5392: f64, t974: f64, t1196: f64, t5398: f64, t3555: f64, t1653: f64, t1735: f64, t3578: f64, t1174: f64, t1726: f64, t1737: f64, t3577: f64, t488: f64, t4889: f64, t4957: f64, t4959: f64, t4994: f64, t4998: f64, t5002: f64, t6158: f64, t6165: f64, t248: f64, t3585: f64, t5971: f64, t1230: f64, t5979: f64, t5975: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6092: f64, t6094: f64, t6096: f64, t6100: f64, t6104: f64, t6108: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6169, t6170, t6177, t6178, t6183, t6184, t6187, t6188, t6191, t6192) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1064(t6168, t68, t484, t3560, t5392, t974, t1196, t5398, t3555, t1653, t1735, t3578);
        let t6197 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1065(t1174, t1726, t1737, t3577, t488, t4889, t4957, t4959, t4994, t4998, t5002, t6158, t6165, t6170, t6178, t6184, t6188, t6192);
        let (t6203, t6207, t6211, t6218) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1066(t248, t3585, t5971, t1230, t5979, t5975, t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108);
    (t6169, t6170, t6177, t6183, t6187, t6191, t6192, t6197, t6203, t6207, t6211, t6218)
}
