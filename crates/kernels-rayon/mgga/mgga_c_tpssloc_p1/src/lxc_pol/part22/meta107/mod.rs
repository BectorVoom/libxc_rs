//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk725;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk726;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk727;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk728;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk729;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk730;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk731;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta107(t2978: f64, t974: f64, t2770: f64, t344: f64, t337: f64, t39: f64, t1887: f64, t60: f64, t976: f64, t984: f64, t343: f64, t883: f64, t607: f64, t2775: f64, t2822: f64, t225: f64, t991: f64, t1008: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2979 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk725(t2978, t974);
        let (t2980, t2986) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk726(t2770, t344, t337, t39, t1887);
        let t2987 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk727(t60, t976);
        let t2988 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk728(t2987, t984);
        let t2989 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk729(t343, t883);
        let t2990 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk730(t2989, t607);
        let (t2994, t3003, t3026) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk731(t2775, t344, t2822, t225, t991);
        let t3030 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk732(t1008, t191);
    (t2979, t2980, t2986, t2987, t2988, t2989, t2990, t2994, t3003, t3026, t3030)
}
