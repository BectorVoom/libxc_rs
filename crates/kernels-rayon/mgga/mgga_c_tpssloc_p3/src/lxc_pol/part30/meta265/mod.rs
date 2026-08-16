//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1204;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1205;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta265(t1307: f64, t1998: f64, t236: f64, t6926: f64, t1995: f64, t6597: f64, t133: f64, t1999: f64, t6600: f64, t1996: f64, t6604: f64, t1339: f64, t1352: f64, t1332: f64, t2002: f64, t559: f64, t1338: f64, t59: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6928, t6929, t6931, t6933, t6935, t6936) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1204(t1307, t1998, t236, t6926, t1995, t6597, t133, t1999, t6600, t1996, t6604);
        let (t6937, t6938, t6940, t6941, t6943) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1205(t1339, t1352, t6936, t1332, t2002, t559, t1338, t59);
        let t6944 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1206(t240, t6943);
    (t6928, t6929, t6931, t6933, t6935, t6936, t6937, t6938, t6940, t6941, t6943, t6944)
}
