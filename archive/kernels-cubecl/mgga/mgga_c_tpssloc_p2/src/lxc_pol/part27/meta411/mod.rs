//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1701;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta411<F: Float>(t3789: F, t5234: F, t3798: F, t1354: F, t12211: F, t5223: F, t1307: F, t210: F, t5226: F, t1810: F, t3719: F, t3804: F, t820: F, t1351: F, t1824: F, t3807: F, t3792: F, t12345: F, t1831: F, t12429: F, t16257: F, t16261: F, t16265: F, t16269: F, t16271: F, t16275: F, t16278: F, t3733: F, t3783: F, t3795: F, t3803: F, t3853: F, t3858: F, t3872: F, t5235: F, t5240: F, t5246: F, t5293: F, t5310: F) -> (F, F, F, F, F, F, F, F) {
        let (t16285, t16290, t16294, t16296, t16300, t16305) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1701::<F>(t3789, t5234, t3798, t1354, t12211, t5223, t1307, t210, t5226, t1810, t3719, t3804, t820);
        let (t16306, t16307, t16308, t16311, t16312, t16313, t16314, t16319) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1702::<F>(t1351, t1824, t3807, t16305, t3792, t1307, t12345, t1831, t12429, t1354, t16257, t16261, t16265, t16269, t16271, t16275, t16278, t16285, t16290, t16294, t16296, t16300, t3733, t3783, t3795, t3803, t3853, t3858, t3872, t5235, t5240, t5246, t5293, t5310);
    (t16306, t16307, t16308, t16311, t16312, t16313, t16314, t16319)
}
