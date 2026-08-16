//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1399;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta260(t10190: f64, t2990: f64, t2986: f64, t2770: f64, t607: f64, t2250: f64, t4510: f64, t2980: f64, t9288: f64, t977: f64, t9258: f64, t978: f64, t3008: f64, t343: f64, t984: f64, t4546: f64, t271: f64, t2775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10191, t10192, t10195) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1399(t10190, t2990, t2986, t2770, t607, t2250);
        let (t10196, t10199, t10200, t10203, t10204, t10208, t10209, t10213) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1400(t10195, t4510, t2980, t9288, t977, t9258, t978, t3008, t343, t984, t4546, t271, t2775);
    (t10191, t10192, t10195, t10196, t10199, t10200, t10203, t10204, t10208, t10209, t10213)
}
