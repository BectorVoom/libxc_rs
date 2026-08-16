//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 413/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk413(t2074: f64, t2126: f64, t1979: f64, t1982: f64, t1986: f64, t1989: f64, t1994: f64, t1996: f64, t1998: f64, t2002: f64, t2003: f64, t2007: f64, t2016: f64, t2019: f64, t2023: f64, t2025: f64, t444: f64, t455: f64, t552: f64) -> (f64, f64) {
    let t2127 = t2074 + t2126;
    let t2129 = 1.8805371096875316_f64 * t1979 * t455 - 3.7610742193750633_f64 * t1982 * t455 - 1.8805371096875316_f64 * t1986 * t552 + 22.07984838129906_f64 * t1989 + t1994 - 5.40024514194619_f64 * t1996 + 5.40024514194619_f64 * t1998 + t2002 - 22.07984838129906_f64 * t2003 - t2007 * t2016 + t2019 + t2023 + t2025 + t444 * t2127;
    (t2127, t2129)
}
