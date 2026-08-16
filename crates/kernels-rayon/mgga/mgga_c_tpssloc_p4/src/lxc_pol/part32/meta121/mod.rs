//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk708;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk709;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk710;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta121(t2770: f64, t344: f64, t337: f64, t39: f64, t1887: f64, t60: f64, t976: f64, t984: f64, t343: f64, t883: f64, t607: f64, t2775: f64, t2822: f64, t225: f64, t991: f64, t1008: f64, t191: f64, t349: f64, t1011: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2980, t2986) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk708(t2770, t344, t337, t39, t1887);
        let t2987 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk709(t60, t976);
        let (t2988, t2989, t2990, t2994, t3003, t3026, t3030) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk710(t2987, t984, t343, t883, t607, t2775, t344, t2822, t225, t991, t1008, t191);
        let (t3031, t3032) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk711(t3030, t349, t1011, t68);
    (t2980, t2986, t2987, t2988, t2989, t2990, t2994, t3003, t3026, t3030, t3031, t3032)
}
