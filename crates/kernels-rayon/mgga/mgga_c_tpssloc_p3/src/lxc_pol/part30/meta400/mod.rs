//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1524;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta400(t17691: f64, t4588: f64, t4582: f64, t14187: f64, t17686: f64, t5878: f64, t884: f64, t3071: f64, t1616: f64, t4347: f64, t376: f64, t5866: f64, t4594: f64, t1023: f64, t1041: f64, t10413: f64, t10436: f64, t10511: f64, t10871: f64, t14049: f64, t14059: f64, t17688: f64, t3039: f64, t3070: f64, t3114: f64, t3130: f64, t4585: f64, t4590: f64, t4644: f64, t5869: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17693, t17697, t17701, t17705, t17712) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1524(t17691, t4588, t4582, t14187, t17686, t5878, t884, t3071, t1616, t4347, t376, t5866);
        let (t17714, t17718, t17725) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1525(t17712, t4594, t4582, t1023, t1041, t10413, t10436, t10511, t10871, t14049, t14059, t17688, t17693, t17697, t17701, t17705, t3039, t3070, t3114, t3130, t4585, t4590, t4644, t5869);
    (t17693, t17697, t17701, t17705, t17714, t17718, t17725)
}
