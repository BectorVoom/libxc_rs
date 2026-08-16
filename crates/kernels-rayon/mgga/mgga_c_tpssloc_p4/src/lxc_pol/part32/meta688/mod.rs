//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta688 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2130;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2131;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta688(t1873: f64, t96709: f64, t5464: f64, t81442: f64, t666: f64, t81446: f64, t1453: f64, t4067: f64, t22473: f64, t22470: f64, t5488: f64, t19529: f64, t6530: f64, t109: f64, t81438: f64, t81440: f64, t86589: f64, t86591: f64, t92121: f64, t1268: f64, t28030: f64, t6535: f64, t26114: f64, t7461: f64, t19994: f64, t24995: f64, t8945: f64, t28831: f64, t83886: f64, t6287: f64, t652: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96711, t96713, t96716, t96719, t96721, t96724, t96726) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2130(t1873, t96709, t5464, t81442, t666, t81446, t1453, t4067, t22473, t22470, t5488, t19529, t6530);
        let (t96729, t96731) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2131(t109, t81438, t81440, t86589, t86591, t92121, t96713, t96716, t96719, t96721, t96724, t96726, t1268);
        let (t96738, t96740, t96746, t96755, t96758) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2132(t28030, t6535, t26114, t7461, t19994, t24995, t8945, t28831, t83886, t6287, t652, t6534);
    (t96711, t96729, t96731, t96738, t96740, t96746, t96755, t96758)
}
