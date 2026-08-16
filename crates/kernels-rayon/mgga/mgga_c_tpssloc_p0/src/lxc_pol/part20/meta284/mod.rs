//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1478;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta284(t2924: f64, t952: f64, t2932: f64, t950: f64, t2836: f64, t914: f64, t2792: f64, t2844: f64, t912: f64, t2842: f64, t2880: f64, t933: f64, t10662: f64, t913: f64, t2860: f64, t919: f64, t2862: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10720, t10723, t10724, t10727, t10729, t10731, t10733, t10734) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1478(t2924, t952, t2932, t950, t2836, t914, t2792, t2844, t912, t2842, t2880, t933);
        let (t10737, t10739, t10740, t10743) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1479(t10662, t913, t2842, t2860, t919, t2862, t931);
    (t10720, t10723, t10724, t10727, t10729, t10731, t10733, t10734, t10737, t10739, t10740, t10743)
}
