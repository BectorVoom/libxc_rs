//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 431/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk431<F: Float>(t1424: F, t1448: F, t1505: F, t1518: F, t1991: F, t1997: F, t1999: F, t2001: F, t2004: F, t2006: F, t2009: F, t2014: F, t2017: F, t2020: F, t2022: F, t2027: F, t2030: F, t2032: F, t213: F) -> (F, F, F, F, F) {
    let t2034 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t1424;
    let t2035 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t1448;
    let t2036 = t1505 / F::cast_from(45.0_f64);
    let t2037 = t1518 / F::cast_from(45.0_f64);
    let t2038 = t1991 + t1997 + t1999 + t2001 + t2004 + t2006 + t2009 + t2014 - t2017 - t2020 + t2022 * t213 / F::cast_from(3.0_f64) + t2027 / F::cast_from(3.0_f64) + F::cast_from(0.06077777777777778_f64) * t2030 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2032 + t2034 + t2035 + t2036 + t2037;
    (t2034, t2035, t2036, t2037, t2038)
}
