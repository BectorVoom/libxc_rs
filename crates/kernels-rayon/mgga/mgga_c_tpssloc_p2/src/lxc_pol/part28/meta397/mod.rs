//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1542;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta397(t3789: f64, t5234: f64, t3798: f64, t1354: f64, t12211: f64, t5223: f64, t1307: f64, t210: f64, t5226: f64, t1810: f64, t3719: f64, t3804: f64, t820: f64, t1351: f64, t1824: f64, t3807: f64, t3792: f64, t12345: f64, t1831: f64, t12429: f64, t16257: f64, t16261: f64, t16265: f64, t16269: f64, t16271: f64, t16275: f64, t16278: f64, t3733: f64, t3783: f64, t3795: f64, t3803: f64, t3853: f64, t3858: f64, t3872: f64, t5235: f64, t5240: f64, t5246: f64, t5293: f64, t5310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16285, t16290, t16294, t16296, t16300, t16305) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1542(t3789, t5234, t3798, t1354, t12211, t5223, t1307, t210, t5226, t1810, t3719, t3804, t820);
        let (t16306, t16307, t16308, t16311, t16312, t16313, t16314, t16319) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1543(t1351, t1824, t3807, t16305, t3792, t1307, t12345, t1831, t12429, t1354, t16257, t16261, t16265, t16269, t16271, t16275, t16278, t16285, t16290, t16294, t16296, t16300, t3733, t3783, t3795, t3803, t3853, t3858, t3872, t5235, t5240, t5246, t5293, t5310);
    (t16306, t16307, t16308, t16311, t16312, t16313, t16314, t16319)
}
