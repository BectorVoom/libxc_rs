//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 413/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk413<F: Float>(t2074: F, t2126: F, t1979: F, t1982: F, t1986: F, t1989: F, t1994: F, t1996: F, t1998: F, t2002: F, t2003: F, t2007: F, t2016: F, t2019: F, t2023: F, t2025: F, t444: F, t455: F, t552: F) -> (F, F) {
    let t2127 = t2074 + t2126;
    let t2129 = F::cast_from(1.8805371096875316_f64) * t1979 * t455 - F::cast_from(3.7610742193750633_f64) * t1982 * t455 - F::cast_from(1.8805371096875316_f64) * t1986 * t552 + F::cast_from(22.07984838129906_f64) * t1989 + t1994 - F::cast_from(5.40024514194619_f64) * t1996 + F::cast_from(5.40024514194619_f64) * t1998 + t2002 - F::cast_from(22.07984838129906_f64) * t2003 - t2007 * t2016 + t2019 + t2023 + t2025 + t444 * t2127;
    (t2127, t2129)
}
