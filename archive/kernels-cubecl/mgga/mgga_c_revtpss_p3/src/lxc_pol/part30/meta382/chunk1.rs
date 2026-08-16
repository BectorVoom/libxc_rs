//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1434/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1434<F: Float>(t1903: F, t4131: F, t4076: F, t4077: F, t9657: F, t1444: F, t5774: F, t10171: F, t13727: F, t13733: F, t13737: F, t1424: F, t1904: F, t9632: F, t9636: F, t9639: F, t9642: F, t9650: F) -> (F, F, F, F) {
    let t13738 = t1903 * t4131;
    let t13739 = t4076 * t13738;
    let t13743 = t9657 * t1903 * t4077;
    let t13746 = t5774 * t1444;
    let t13747 = t4076 * t13746;
    let t13750 = F::cast_from(0.14634331517634470219e-1_f64) * t9632 - F::cast_from(0.54878743191129263322e-2_f64) * t9636 + t9639 - F::cast_from(0.13009920719177044025e-2_f64) * t9642 + t9650 - F::cast_from(0.65854491829355115987e0_f64) * t10171 * t1904 - F::cast_from(0.65049603595885220126e-3_f64) * t13727 - t13733 - t13737 + F::cast_from(0.13170898365871023197e1_f64) * t1424 * t13739 - F::cast_from(0.39512695097613069591e1_f64) * t1424 * t13743 + F::cast_from(0.26341796731742046394e1_f64) * t1424 * t13747;
    (t13739, t13743, t13747, t13750)
}
