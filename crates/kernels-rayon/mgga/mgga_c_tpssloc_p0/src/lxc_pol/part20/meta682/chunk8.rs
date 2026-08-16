//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2582/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2582(t50853: f64, t43768: f64, t43770: f64, t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t44466: f64, t50824: f64, t50846: f64, t50848: f64, t50851: f64, t50859: f64, t50863: f64, t50867: f64, t50871: f64, t50875: f64, t50881: f64, t50886: f64) -> f64 {
    let t52313 = 5.0_f64 / 9.0_f64 * t50853;
    let t52327 = -3.0_f64 * t50824 + 40.0_f64 / 81.0_f64 * t50846 + t50848 / 3.0_f64 - t50851 / 6.0_f64 - t52313 - t43768 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t43770 - t44466 + t50859 / 18.0_f64 + 2.0_f64 * t50863 - t50867 - 3.0_f64 * t50871 - t50875 / 3.0_f64 - 4.0_f64 * t50881 + t50886 / 6.0_f64 - 2.0_f64 / 9.0_f64 * t43835 + 2.0_f64 / 3.0_f64 * t43837 + t43839 / 9.0_f64 + 5.0_f64 / 27.0_f64 * t43855 + 4.0_f64 / 81.0_f64 * t43857;
    t52327
}
