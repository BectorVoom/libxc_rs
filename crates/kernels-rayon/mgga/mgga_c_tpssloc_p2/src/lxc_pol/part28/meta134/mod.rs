//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk731;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk732;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta134(t2988: f64, t2990: f64, t2775: f64, t344: f64, t2244: f64, t977: f64, t2250: f64, t978: f64, t2822: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64, t340: f64, t343: f64, t974: f64, t984: f64, t2955: f64, t2958: f64, t2960: f64, t2969: f64, t2972: f64, t2975: f64, t2982: f64, t2986: f64, t346: f64, t973: f64, t980: f64, t987: f64, t381: f64, t1049: f64, t990: f64, t225: f64, t991: f64, t1008: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2991, t2995, t2996, t2999, t3000, t3003, t3008) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk731(t2988, t2990, t2775, t344, t2244, t977, t2250, t978, t2822, t2824, t2828, t2831, t2834);
        let (t3010, t3014, t3016, t3020) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk732(t3008, t340, t343, t974, t984, t2955, t2958, t2960, t2969, t2972, t2975, t2982, t2986, t2991, t2996, t3000, t346, t973, t980, t987);
        let (t3021, t3023, t3026, t3030) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk733(t3020, t381, t1049, t990, t225, t991, t1008, t191);
    (t2995, t2999, t3003, t3008, t3010, t3014, t3016, t3020, t3021, t3023, t3026, t3030)
}
