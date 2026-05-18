//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 412/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk412<F: Float>(t2115: F, t93: F, t1993: F, t2001: F, t1732: F, t1989: F, t1996: F, t1998: F, t2003: F, t2085: F, t2088: F, t2091: F, t2104: F, t2108: F, t2110: F, t2111: F, t2114: F, t455: F) -> (F, F, F, F) {
    let t2116 = t93 * t2115;
    let t2121 = F::new(0.037002892246025966) * t1993;
    let t2124 = F::new(0.14975624337724558) * t2001;
    let t2126 = -t2085 * t455 / F::new(6.0) - t2088 * t455 / F::new(6.0) - t2091 * t455 / F::new(6.0) - t2104 * t455 / F::new(6.0) - t2108 + t2110 + t2111 * t455 / F::new(6.0) + t2114 * t2116 / F::new(12.0) + F::new(0.10237773105191754) * t1732 - F::new(0.14975624337724558) * t1989 - t2121 + F::new(0.037002892246025966) * t1996 - F::new(0.037002892246025966) * t1998 - t2124 + F::new(0.14975624337724558) * t2003;
    (t2116, t2121, t2124, t2126)
}
