//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1059;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta234<F: Float>(t6168: F, t68: F, t484: F, t3560: F, t5392: F, t974: F, t1196: F, t5398: F, t3555: F, t1653: F, t1735: F, t3578: F, t1174: F, t1726: F, t1737: F, t3577: F, t488: F, t4889: F, t4957: F, t4959: F, t4994: F, t4998: F, t5002: F, t6158: F, t6165: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6169, t6170, t6177, t6178, t6183, t6184, t6187, t6188, t6191, t6192) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1059::<F>(t6168, t68, t484, t3560, t5392, t974, t1196, t5398, t3555, t1653, t1735, t3578);
        let t6197 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1060::<F>(t1174, t1726, t1737, t3577, t488, t4889, t4957, t4959, t4994, t4998, t5002, t6158, t6165, t6170, t6178, t6184, t6188, t6192);
    (t6169, t6170, t6177, t6178, t6183, t6184, t6187, t6188, t6191, t6192, t6197)
}
