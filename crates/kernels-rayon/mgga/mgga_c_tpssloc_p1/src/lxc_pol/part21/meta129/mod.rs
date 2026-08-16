//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk867;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta129(t3040: f64, t3131: f64, t1021: f64, t248: f64, t135: f64, t999: f64, t973: f64, t2250: f64, t998: f64, t974: f64, t2770: f64, t2978: f64, t2244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3132 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk867(t3040, t3131);
        let (t3134, t3139, t3140, t3142, t3143, t3146, t3147) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk868(t1021, t248, t3132, t135, t999, t973, t2250, t998, t974, t2770, t2978, t2244);
    (t3132, t3134, t3139, t3140, t3142, t3143, t3146, t3147)
}
