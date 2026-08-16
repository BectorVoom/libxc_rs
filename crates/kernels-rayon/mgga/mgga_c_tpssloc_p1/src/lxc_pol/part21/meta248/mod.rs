//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1451;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta248(t479: f64, t6163: f64, t471: f64, t225: f64, t6150: f64, t68: f64, t484: f64, t3560: f64, t5392: f64, t974: f64, t1196: f64, t5398: f64, t3555: f64, t1653: f64, t1735: f64, t3578: f64, t1174: f64, t1726: f64, t1737: f64, t3577: f64, t488: f64, t4889: f64, t4957: f64, t4959: f64, t4994: f64, t4998: f64, t5002: f64, t6158: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6164, t6165, t6168, t6169, t6170, t6177, t6178, t6183) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1451(t479, t6163, t471, t225, t6150, t68, t484, t3560, t5392, t974, t1196, t5398);
        let (t6187, t6191, t6192, t6197) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1452(t6183, t974, t3555, t5392, t1653, t1735, t3578, t1174, t1726, t1737, t3577, t488, t4889, t4957, t4959, t4994, t4998, t5002, t6158, t6165, t6170, t6178);
    (t6164, t6165, t6168, t6169, t6170, t6177, t6183, t6187, t6191, t6192, t6197)
}
