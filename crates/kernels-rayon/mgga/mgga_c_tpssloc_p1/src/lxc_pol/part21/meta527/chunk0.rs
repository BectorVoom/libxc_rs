//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2182/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2182(t10523: f64, t5774: f64, t4497: f64, t959: f64, t4472: f64, t4488: f64, t2929: f64, t5790: f64, t17490: f64, t17504: f64, t17506: f64, t17509: f64, t17512: f64, t17515: f64, t17519: f64, t17523: f64, t17526: f64, t17530: f64, t17936: f64, t17940: f64, t17942: f64, t17944: f64, t17946: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17947 = t10523 * t5774;
    let t17948 = t17947 * t4497;
    let t17950 = 0.10389515463408878255e3_f64 * t959 * t17948;
    let t17951 = t4488 * t4472;
    let t17953 = 0.23392894490538584828e1_f64 * t959 * t17951;
    let t17954 = t2929 * t5790;
    let t17955 = t17954 * t4497;
    let t17957 = 0.17315859105681463759e2_f64 * t959 * t17955;
    let t17958 = -t17936 - t17490 + t17940 - t17942 - t17944 + t17946 + t17950 + t17953 - t17504 + t17506 + t17509 - t17512 - t17515 - t17519 + t17523 + t17526 + t17530 - t17957;
    (t17947, t17948, t17950, t17951, t17953, t17954, t17955, t17957, t17958)
}
