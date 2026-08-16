//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1158;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta360(t10277: f64, t976: f64, t11046: f64, t42387: f64, t10457: f64, t820: f64, t10969: f64, t121: f64, t10213: f64, t41687: f64, t1043: f64, t204: f64, t340: f64, t625: f64, t221: f64, t339: f64, t344: f64, t343: f64, t42308: f64, t974: f64, t41666: f64, t2978: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42444, t42483, t42488, t42592, t42624, t42749) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1158(t10277, t976, t11046, t42387, t10457, t820, t10969, t121, t10213, t41687, t1043, t204);
        let (t42813, t42817, t42841, t42861, t42862, t42875) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1159(t340, t625, t221, t339, t344, t10277, t343, t42308, t974, t41666, t2978, t698);
    (t42444, t42483, t42488, t42592, t42624, t42749, t42813, t42817, t42841, t42861, t42862, t42875)
}
