//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 465/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk465<F: Float>(t1981: F, t509: F, t552: F, t557: F, t303: F, t1471: F, t1472: F, t1650: F, t1477: F, t1897: F, t542: F, t1482: F, t1961: F, t1102: F, t1470: F, t1924: F, t344: F, t486: F) -> (F, F, F, F, F, F, F, F) {
    let t1982 = t509 * t1981;
    let t1983 = t1982 * t552;
    let t1984 = t1983 * t557;
    let t1985 = t303 * t1984;
    let t1988 = t1471 * t1472 * t1650;
    let t1991 = t1477 * t1897;
    let t1992 = t542 * t1991;
    let t1995 = t1482 * t1961;
    let t1996 = t542 * t1995;
    let t2001 = t1470 + 0.65704296666666666667e-3 * t1102 * t1988 + 0.1478346675e-2 * t344 * t1992 - 0.98556445e-3 * t344 * t1996 - 4.0 * t486 * t1924;
    (t1984, t1985, t1988, t1991, t1992, t1995, t1996, t2001)
}
