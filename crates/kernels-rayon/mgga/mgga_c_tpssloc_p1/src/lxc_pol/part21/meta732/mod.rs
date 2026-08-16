//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2589;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta732(t44620: f64, t461: f64, t60: f64, t15394: f64, t1714: f64, t3439: f64, t3447: f64, t4724: f64, t697: f64, t11590: f64, t15376: f64, t11554: f64, t1706: f64, t44579: f64, t4904: f64, t11545: f64, t134: f64, t14726: f64, t11579: f64, t15338: f64, t4899: f64, t4928: f64, t11570: f64, t12648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52096, t52100, t52109, t52122, t52124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2589(t44620, t461, t60, t15394, t1714, t3439, t3447, t4724, t697, t11590, t15376, t11554, t1706);
        let (t52127, t52133, t52135, t52138, t52140, t52161) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2590(t3447, t44579, t4904, t11545, t134, t461, t14726, t11579, t15338, t4899, t4928, t11570, t12648);
    (t52096, t52100, t52109, t52122, t52124, t52127, t52133, t52135, t52138, t52140, t52161)
}
