//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 413/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk413<F: Float>(t2074: F, t2126: F, t1979: F, t1982: F, t1986: F, t1989: F, t1994: F, t1996: F, t1998: F, t2002: F, t2003: F, t2007: F, t2016: F, t2019: F, t2023: F, t2025: F, t444: F, t455: F, t552: F) -> (F, F) {
    let t2127 = t2074 + t2126;
    let t2129 = F::new(1.8805371096875316) * t1979 * t455 - F::new(3.7610742193750633) * t1982 * t455 - F::new(1.8805371096875316) * t1986 * t552 + F::new(22.07984838129906) * t1989 + t1994 - F::new(5.40024514194619) * t1996 + F::new(5.40024514194619) * t1998 + t2002 - F::new(22.07984838129906) * t2003 - t2007 * t2016 + t2019 + t2023 + t2025 + t444 * t2127;
    (t2127, t2129)
}
