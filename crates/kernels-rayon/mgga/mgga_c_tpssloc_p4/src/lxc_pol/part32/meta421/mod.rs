//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1625;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta421(t1246: f64, t19128: f64, t5079: f64, t6256: f64, t3625: f64, t5011: f64, t1755: f64, t5068: f64, t1235: f64, t6224: f64, t1215: f64, t475: f64, t6739: f64, t6252: f64, t11889: f64, t6260: f64, t11888: f64, t11904: f64, t11907: f64, t11914: f64, t1244: f64, t15027: f64, t15032: f64, t15245: f64, t1756: f64, t19123: f64, t3604: f64, t3610: f64, t3624: f64, t5064: f64, t5069: f64, t5080: f64, t5084: f64, t6253: f64, t6261: f64, t6263: f64) -> (f64, f64, f64, f64, f64) {
        let (t19129, t19131, t19138, t19139, t19142, t19145, t19146, t19153) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1625(t1246, t19128, t5079, t6256, t3625, t5011, t1755, t5068, t1235, t6224, t1215, t475, t6739);
        let (t19156, t19164) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1626(t19153, t6252, t11889, t1215, t5079, t6260, t11888, t11904, t11907, t11914, t1244, t15027, t15032, t15245, t1756, t19123, t19129, t19131, t19139, t19142, t19146, t3604, t3610, t3624, t5064, t5069, t5080, t5084, t6253, t6261, t6263);
    (t19138, t19145, t19153, t19156, t19164)
}
