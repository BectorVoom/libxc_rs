//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2356/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2356(t48338: f64, t10263: f64, t4528: f64, t12606: f64, t2989: f64, t10241: f64, t13861: f64, t1600: f64, t2986: f64, t2988: f64, t3008: f64, t3014: f64, t343: f64, t42554: f64, t43061: f64, t4514: f64, t4540: f64, t4543: f64, t4546: f64, t48329: f64, t48336: f64, t973: f64) -> f64 {
    let t48339 = 0.14814814814814814814e-2_f64 * t48338;
    let t48342 = t10263 * t4528;
    let t48357 = t2989 * t12606;
    let t48361 = t48329 - 0.24999999999999999999e-2_f64 * t973 * t4546 * t4540 * t3008 * t343 - 0.3086419753086419753e-3_f64 * t48336 - t48339 + 0.38024691358024691358e-1_f64 * t42554 * t1600 - 0.81481481481481481478e-2_f64 * t48342 - 0.24444444444444444444e-1_f64 * t10263 * t4543 - 0.24999999999999999999e-2_f64 * t973 * t4546 * t4540 * t3014 * t343 - 0.27777777777777777777e-3_f64 * t2986 * t43061 * t4514 - 0.83333333333333333331e-3_f64 * t2986 * t10241 * t13861 - 0.83333333333333333331e-3_f64 * t2986 * t2988 * t48357;
    t48361
}
