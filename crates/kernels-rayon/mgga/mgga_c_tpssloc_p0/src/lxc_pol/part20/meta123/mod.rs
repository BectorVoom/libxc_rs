//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk805;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta123(t2988: f64, t2990: f64, t2775: f64, t344: f64, t2244: f64, t977: f64, t2250: f64, t978: f64, t2822: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64, t340: f64, t343: f64, t974: f64, t984: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2991, t2995, t2996, t2999, t3000, t3003, t3008) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk805(t2988, t2990, t2775, t344, t2244, t977, t2250, t978, t2822, t2824, t2828, t2831, t2834);
        let (t3010, t3011, t3014) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk806(t3008, t340, t343, t974, t984);
    (t2991, t2995, t2996, t2999, t3000, t3003, t3008, t3010, t3011, t3014)
}
