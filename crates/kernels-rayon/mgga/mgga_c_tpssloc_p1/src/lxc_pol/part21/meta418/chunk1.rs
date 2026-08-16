//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1937/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1937(t14956: f64, t423: f64, t1254: f64, t14696: f64, t14701: f64, t14833: f64, t14835: f64, t14837: f64, t14840: f64, t14844: f64, t14847: f64, t14849: f64, t14852: f64, t14857: f64, t14860: f64, t14862: f64, t14864: f64, t14866: f64, t14916: f64, t14936: f64, t14939: f64, t4700: f64) -> (f64, f64) {
    let t14958 = 0.621814e-1_f64 * t14956 * t423;
    let t14959 = -2.0_f64 * t1254 * t14696 * t4700 + t14701 - t14833 - t14835 - t14837 - t14840 + t14844 + t14847 + t14849 + t14852 - t14857 - t14860 - t14862 + t14864 + t14866 + t14916 + t14936 + t14939 - t14958;
    (t14958, t14959)
}
